use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use regex::Regex;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

pub type SshTunnel = Arc<Mutex<Child>>;
pub type SshHandle = thread::JoinHandle<(SshStatus, ExitStatus)>;

pub fn ssh_watch_loop<F>(tunnel: SshTunnel, exit_callback: F) -> (SshStatus, ExitStatus)
where
    F: FnOnce(SshStatus) + Send,
{
    let exit_status: ExitStatus;

    loop {
        let mut tunnel = tunnel.lock().expect("failed to lock tunnel");
        match tunnel.try_wait() {
            Ok(Some(status)) => {
                println!("exited with {status}");
                exit_status = if let Some(code) = status.code() {
                     FromPrimitive::from_i32(code).unwrap()
                } else {
                    ExitStatus::Canceled
                };
                break;
            }
            Ok(None) => (),
            Err(e) => {
                println!("error attempting to wait: {e}");
                exit_status = ExitStatus::ProcError;
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    // Capture stderr to discover exit reason
    let mut stderr = {
        tunnel.lock().unwrap().stderr.take().unwrap()
    };

    let mut err_msg = String::new();
    stderr.read_to_string(&mut err_msg).unwrap();

    println!("Error: {}", err_msg);
    let ssh_status = parse_stderr(&err_msg);
    exit_callback(ssh_status.clone());
    (ssh_status, exit_status)
}

pub fn start_ssh_tunnel(config: SshConfig) -> Result<SshTunnel, SshStatus> {
    let mut proc = Command::new("ssh")
        .args(config.to_args())
        .env_clear()
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().map_err(|err| {
            SshStatus::ProcError(err.to_string())
        })?;


    // A few bytes will be printed to stdout once the login is complete. Wait for that output,
    // or throw an error if it fails to happen
    let mut stdout = proc.stdout.take().unwrap();
    let mut buffer = [0; 15];
    let len = stdout.read(&mut buffer).map_err(|err| {
        SshStatus::ProcError(err.to_string())
    })?;

    if len < 15 {
        let mut stderr = proc.stderr.take().unwrap();
        let mut err_msg = String::new();
        stderr.read_to_string(&mut err_msg).unwrap();
        Err(parse_stderr(&err_msg))
    } else {
        let tunnel = Arc::new(Mutex::new(proc));
        Ok(tunnel)
    }
}

pub fn start_and_watch_ssh_tunnel<F>(
    config: SshConfig,
    callback: F,
) -> Result<(SshTunnel, SshHandle), SshStatus>
where
    F: FnOnce(SshStatus) + Send + 'static,
{
    let tunnel = start_ssh_tunnel(config)?;
    let watched_tunnel = tunnel.clone();
    let handle = std::thread::spawn(move || ssh_watch_loop(watched_tunnel, callback));

    Ok((tunnel, handle))
}

fn parse_stderr(msg: &str) -> SshStatus {
    if check_dropped(msg) {
        SshStatus::Dropped
    } else if check_unreachable(msg) {
        SshStatus::Unreachable
    } else if check_denied(msg) {
        SshStatus::Denied
    } else if check_bad_port(msg) {
        SshStatus::ConfigError(msg.to_string())
    } else {
        SshStatus::Exited
    }
}

fn check_dropped(msg: &str) -> bool {
    let re = Regex::new("Timeout, server .* not responding").unwrap();
    re.is_match(msg)
}

fn check_unreachable(msg: &str) -> bool {
    msg.contains("Network is unreachable")
}

fn check_denied(msg: &str) -> bool {
    msg.contains("Permission denied")
}

fn check_bad_port(msg: &str) -> bool {
    msg.contains("Bad local forwarding specification")
}

#[derive(Debug, Clone)]
pub enum SshStatus {
    Connected,
    Dropped,
    Unreachable,
    Denied,
    Exited,
    Retrying,
    ConfigError(String),
    ProcError(String)
}

impl SshStatus {
    pub fn to_signal(&self) -> String {
        let status = match self {
            SshStatus::Connected => "CONNECTED",
            SshStatus::Dropped => "DROPPED",
            SshStatus::Unreachable => "UNREACHABLE",
            SshStatus::Denied => "DENIED",
            SshStatus::Exited => "EXIT",
            SshStatus::Retrying => "RETRYING",
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

#[derive(FromPrimitive)]
pub enum ExitStatus {
    Clean = 0,
    ProcError = 1,
    Canceled = 2,
    SshError = 255
}

#[derive(Debug, Clone)]
pub struct SshConfig {
    end_host: String,
    username: String,
    key_path: String,
    to_host: String,
    local_port: u32,
    remote_port: u32,
    keepalive: u32,
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
        flags: &[&str]
    ) -> Self {
        SshConfig {
            end_host: String::from(end_host),
            username: String::from(username),
            key_path: String::from(key_path),
            to_host: String::from(to_host),
            local_port,
            remote_port,
            keepalive,
            flags: flags.iter().map(|f| {f.to_string()}).collect()
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = self.flags.clone();
        args.append(&mut vec![
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
        ].iter().map(|a| {
            a.to_string()
        }).collect::<Vec<String>>());
        
        args
    }
}

#[cfg(test)]
mod tests {
    use super::SshConfig;

    #[test]
    fn test_config() {
        let config = SshConfig::new("endhost", "username", "keypath", "tohost", 1, 2, 10, &["-T"]);
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
