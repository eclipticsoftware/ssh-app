use std::io::Read;
use std::process;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use std::fmt;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::Regex;

/// Defines the necessary interface that a child process type must support to be used by the tunnel library
pub trait ChildProc {
    fn new(config: SshConfig) -> Result<SshTunnel<Self>, SshStatus>;

    /// Waits for the child process to start (or fail to start)
    fn wait_for_start(&mut self) -> Option<SshStatus>;

    /// Checks whether the process has exited. If it has, then it returns the ExitCondition
    fn exited(&mut self) -> Option<ExitCondition>;

    /// Captures the exit reason from the process and returns the corresponding SshStatus
    fn exit_status(&mut self) -> SshStatus;

    /// Kills the process
    fn kill(&mut self);
}

/// A thread-safe wrapper for a tunnel process
pub type SshTunnel<T> = Arc<Mutex<T>>;

/// A process-watching thread handle
pub type SshHandle = thread::JoinHandle<(SshStatus, ExitCondition)>;

/// Starts a tunnel process and waits for the tunnel to connect (or fail)
///
/// # Returns
///
/// Returns a result with the tunnel process if it was successfully spawned, and with a status
/// giving the reason for the failure if not.
pub fn start_wait_ssh_tunnel<T>(config: SshConfig) -> Result<SshTunnel<T>, SshStatus>
where
    T: ChildProc,
{
    let tunnel = T::new(config)?;
    let mut child = tunnel
        .lock()
        .map_err(|err| SshStatus::ProcError(format!("Failed to lock tunnel: {err}")))?;
    if child.wait_for_start().is_some() {
        Err(child.exit_status())
    } else {
        Ok(tunnel.clone())
    }
}

/// Starts a tunnel process and captures the start status in a callback
///
/// # Returns
///
/// Returns a result with the tunnel process if it was successfully spawned, and with a status
/// giving the reason for the failure if not.
pub fn start_ssh_tunnel<T, F>(
    config: SshConfig,
    status_callback: Arc<Mutex<F>>,
) -> Result<SshTunnel<T>, SshStatus>
where
    T: ChildProc + Send + 'static,
    F: FnMut(SshStatus) + Send + 'static,
{
    let tunnel = T::new(config)?;
    let tunnel_sts = tunnel.clone();

    thread::spawn(move || {
        let mut child = match tunnel_sts.lock() {
            Ok(child) => child,
            Err(err) => {
                call_status_callback(
                    status_callback,
                    SshStatus::ProcError(format!("Failed to lock tunnel: {err}")),
                );
                return;
            }
        };

        if let Some(status) = child.wait_for_start() {
            call_status_callback(status_callback, status)
        } else {
            call_status_callback(status_callback, SshStatus::Connected)
        }
    });

    Ok(tunnel)
}

/// Watches a tunnel process and calls the given callback when it exits
///
/// # Returns
///
/// Returns a tuple containing the SshStatus and the ExitCondition
pub fn ssh_watch_loop<T, F>(
    tunnel: SshTunnel<T>,
    exit_callback: Arc<Mutex<F>>,
) -> (SshStatus, ExitCondition)
where
    T: ChildProc,
    F: FnMut(SshStatus) + Send,
{
    loop {
        match tunnel.lock() {
            Ok(mut tunnel) => {
                if let Some(exit_cond) = tunnel.exited() {
                    let ssh_status = tunnel.exit_status();
                    call_status_callback(exit_callback, ssh_status.clone());
                    return (ssh_status, exit_cond);
                }
            }
            Err(err) => {
                let ssh_status = SshStatus::ProcError(format!("Failed to lock tunnel: {err}"));
                call_status_callback(exit_callback, ssh_status.clone());
                return (ssh_status, ExitCondition::ProcError);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/// Starts a tunnel process and a watcher thread
///
/// # Returns
///
/// If the spawn was successful, it returns a tuple containing the SshStatus and the ExitCondition.
/// Otherwise it returns a status giving the reason for the failure.
pub fn start_and_watch_ssh_tunnel<T, F>(
    config: SshConfig,
    callback: Arc<Mutex<F>>,
    wait: bool,
) -> Result<(SshTunnel<T>, SshHandle), SshStatus>
where
    T: ChildProc + Send + 'static,
    F: FnMut(SshStatus) + Send + 'static,
{
    let tunnel = if wait {
        start_wait_ssh_tunnel(config)
    } else {
        start_ssh_tunnel(config, callback.clone())
    }?;
    let watched_tunnel = tunnel.clone();
    let handle = std::thread::spawn(move || ssh_watch_loop(watched_tunnel, callback));

    Ok((tunnel, handle))
}

fn call_status_callback<F>(status_callback: Arc<Mutex<F>>, status: SshStatus)
where
    F: FnMut(SshStatus) + Send,
{
    match status_callback.lock() {
        Ok(mut cb) => cb(status),
        Err(err) => println!("Failed to get exit callback handle: {err}"),
    }
}

/// Wraps the standard process::Child struct
pub struct TunnelChild {
    child: process::Child,
}

impl ChildProc for TunnelChild {
    #[cfg(not(target_os = "windows"))]
    fn new(config: SshConfig) -> Result<SshTunnel<Self>, SshStatus> {
        let child = process::Command::new("ssh")
            .args(config.to_args())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .map_err(|err| SshStatus::ProcError(err.to_string()))?;

        Ok(Arc::new(Mutex::new(TunnelChild { child })))
    }

    #[cfg(target_os = "windows")]
    fn new(config: SshConfig) -> Result<SshTunnel<Self>, SshStatus> {
        let mut cmd = process::Command::new("ssh");

        for arg in config.to_args() {
            cmd.raw_arg(arg);
        }

        let child = cmd
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .map_err(|err| SshStatus::ProcError(err.to_string()))?;

        Ok(Arc::new(Mutex::new(TunnelChild { child })))
    }

    // A few bytes will be printed to stdout once the login is complete. Wait for that output,
    // or throw an error if it fails to happen
    fn wait_for_start(&mut self) -> Option<SshStatus> {
        let mut stdout = if let Some(stdout) = self.child.stdout.take() {
            stdout
        } else {
            return Some(SshStatus::ProcError(
                "Failed to capture stdout of ssh process".to_string(),
            ));
        };
        let mut buffer = [0; 15];
        let len = match stdout.read(&mut buffer) {
            Err(err) => return Some(SshStatus::ProcError(err.to_string())),
            Ok(len) => len,
        };
        if len >= 15 {
            None
        } else {
            Some(SshStatus::Disconnected)
        }
    }

    fn exited(&mut self) -> Option<ExitCondition> {
        match self.child.try_wait() {
            Ok(Some(status)) => {
                println!("exited with {status}");
                if let Some(code) = status.code() {
                    match FromPrimitive::from_i32(code) {
                        Some(cond) => Some(cond),
                        None => {
                            println!("Unknown exit code: {code}");
                            Some(ExitCondition::ProcError)
                        }
                    }
                } else {
                    Some(ExitCondition::Canceled)
                }
            }
            Ok(None) => None,
            Err(e) => {
                println!("error attempting to wait: {e}");
                Some(ExitCondition::ProcError)
            }
        }
    }

    //Capture stderr to discover exit reason
    fn exit_status(&mut self) -> SshStatus {
        let mut stderr = if let Some(stderr) = self.child.stderr.take() {
            stderr
        } else {
            return SshStatus::ProcError("Failed to capture stderr of ssh process".to_string());
        };

        let mut err_msg = String::new();
        if stderr.read_to_string(&mut err_msg).is_err() {
            SshStatus::ProcError("Failed to read from stderr".to_string())
        } else {
            println!("Error: {err_msg}");
            SshStatus::from_stderr(&err_msg)
        }
    }

    fn kill(&mut self) {
        match self.child.kill() {
            Ok(_) => {
                //println!("killed");
            }
            Err(_err) => {
                //println!("Not killed: {err}")
            }
        }
    }
}

/// Defines the set of statuses that ssh tunnel can have
#[derive(Debug, Clone)]
pub enum SshStatus {
    /// The tunnel is disconnected. It has either never connected, or it has disconnected cleanly
    Disconnected,

    /// The tunnel is connecting
    Connecting,

    /// The tunnel is connected
    Connected,

    /// The server is unreachable
    Unreachable,

    /// The server has denied access
    Denied,

    /// The tunnel has dropped
    Dropped,

    /// The tunnel is trying to reconnect
    Reconnecting,

    /// There was an error with the configuration
    ConfigError(String),

    /// There was an error with the process
    ProcError(String),
}

fn stderr_is_dropped(msg: &str) -> bool {
    let re = Regex::new("Timeout, server .* not responding")
        .expect("This should not happen: invalid regex expression");

    re.is_match(msg) || msg.contains("Connection reset")
}

fn stderr_is_unreachable(msg: &str) -> bool {
    let re = Regex::new("Connection to .+ port .+ timed out")
        .expect("This should not happen: invalid regex expression");

    re.is_match(msg) || msg.contains("Network is unreachable") || msg.contains("Unknown error")
}

impl SshStatus {
    /// Parses the stderr captured during the ssh process and parses it into an SshStatus
    pub fn from_stderr(msg: &str) -> Self {
        if stderr_is_dropped(msg) {
            SshStatus::Dropped
        } else if stderr_is_unreachable(msg) {
            SshStatus::Unreachable
        } else if msg.contains("Permission denied") {
            SshStatus::Denied
        } else if msg.contains("Bad local forwarding specification") {
            SshStatus::ConfigError(msg.to_string())
        } else {
            SshStatus::Disconnected
        }
    }

    /// Converts the status to a "signal" string for status event signaling
    pub fn to_signal(&self) -> String {
        let status = match self {
            SshStatus::Disconnected => "EXIT",
            SshStatus::Connecting => "CONNECTING",
            SshStatus::Connected => "CONNECTED",
            SshStatus::Unreachable => "UNREACHABLE",
            SshStatus::Denied => "DENIED",
            SshStatus::Dropped => "DROPPED",
            SshStatus::Reconnecting => "RETRYING",
            SshStatus::ConfigError(msg) => {
                println!("Config error: {msg}");
                "BAD_CONFIG"
            }
            SshStatus::ProcError(msg) => {
                println!("Process error: {msg}");
                "ERROR"
            }
        };
        status.to_string()
    }
}

impl fmt::Display for SshStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Defines the set of exit conditions for the tunnel process
#[derive(FromPrimitive, Debug)]
pub enum ExitCondition {
    /// The tunnel exited cleanly. This actualy will only happen if something
    /// goes wrong. A successful tunnel must be killed, which will result in
    /// an SshError or a Canceled condition.
    Clean = 0,

    /// An error occured while handling the tunnel's child process
    ProcError = 1,

    /// The code that the ssh command will return if any error is encountered
    SshError = 255,

    /// No exit code was returned, probably because the process was canceled
    Canceled = -1,
}

/// Configuration parameters for the ssh tunnel
#[derive(Debug, Clone)]
pub struct SshConfig {
    /// The end host (most likely an ip address)
    end_host: String,

    /// The username to log in with
    username: String,

    /// A path to the key file to use. This must not be password encrypted
    key_path: String,

    /// The host to forward the tunnel to (probably `localhost`)
    to_host: String,

    /// The port to use on the to host
    local_port: u32,

    /// The port to use on the end host
    remote_port: u32,

    /// The keepalive time (in seconds). If the connection is interrupted for longer than this interval,
    /// the tunnel process will exit
    keepalive: u32,

    /// Any additional flags required
    flags: Vec<String>,
}

impl SshConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        end_host: &str,
        username: &str,
        key_path: &str,
        to_host: &str,
        local_port: u32,
        remote_port: u32,
        keepalive: u32,
        flags: &[&str],
    ) -> Self {
        let kp = key_path.to_string().replace("C:", "").replace('\\', "/");
        SshConfig {
            end_host: String::from(end_host),
            username: String::from(username),
            key_path: kp,
            to_host: String::from(to_host),
            local_port,
            remote_port,
            keepalive,
            flags: flags.iter().map(|f| f.to_string()).collect(),
        }
    }

    /// Converts the config object to an argument vector
    pub fn to_args(&self) -> Vec<String> {
        let mut args = self.flags.clone();
        args.append(
            &mut vec![
                "-o",
                "UserKnownHostsFile=/dev/null",
                "-o",
                "StrictHostKeyChecking=no",
                "-o",
                "ServerAliveCountMax=1",
                "-o",
                &format!("ServerAliveInterval={}", self.keepalive),
                "-L",
                &format!("{}:{}:{}", self.local_port, self.to_host, self.remote_port),
                "-i",
                &self.key_path,
                &format!("{}@{}", self.username, self.end_host),
            ]
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>(),
        );

        println!("Args: {:?}", args);
        args
    }
}

#[cfg(test)]
mod tests {
    use super::SshConfig;

    #[test]
    fn test_config() {
        let config = SshConfig::new(
            "endhost",
            "username",
            "keypath",
            "tohost",
            1,
            2,
            10,
            &["-T"],
        );
        let args = config.to_args();
        let expected: Vec<String> = vec![
            "-T",
            "-o",
            "UserKnownHostsFile=/dev/null",
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "ServerAliveCountMax=1",
            "-o",
            "ServerAliveInterval=10",
            "-L",
            "1:tohost:2",
            "-i",
            "keypath",
            "username@endhost",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();
        println!("{:?}", args);
        assert!(args == expected);
    }
}
