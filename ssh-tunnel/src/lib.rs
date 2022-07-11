use std::io::{Read, Result};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use regex::Regex;

pub type SshTunnel = Arc<Mutex<Child>>;

pub fn ssh_watch_loop<F>(tunnel: SshTunnel, callback: F) -> (SshStatus, i32)
where
    F: FnOnce(SshStatus) + Send,
{
    let mut exit_status = 0;
    let mut stderr;
    {
        stderr = tunnel.lock().unwrap().stderr.take().unwrap();
    }

    loop {
        let mut tunnel = tunnel.lock().unwrap();
        match tunnel.try_wait() {
            Ok(Some(status)) => {
                println!("exited with {status}");
                exit_status = status.code().unwrap();
                break;
            }
            Ok(None) => (),
            Err(e) => {
                println!("error attempting to wait: {e}");
                exit_status = 100;
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    let mut err_msg = String::new();
    stderr.read_to_string(&mut err_msg).unwrap();

    println!("Error: {}", err_msg);
    let ssh_status = parse_stderr(&err_msg);
    callback(ssh_status.clone());
    (ssh_status, exit_status)
}

pub fn start_ssh_tunnel(config: SshConfig) -> Result<SshTunnel> {
    let proc = Command::new("ssh")
        .args(config.to_args())
        .stderr(Stdio::piped())
        .spawn()?;

    let tunnel = Arc::new(Mutex::new(proc));
    Ok(tunnel)
}

pub fn start_and_watch_ssh_tunnel<F>(
    config: SshConfig,
    callback: F,
) -> Result<(SshTunnel, thread::JoinHandle<(SshStatus, i32)>)>
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

#[derive(Debug, Clone)]
pub enum SshStatus {
    Running,
    Dropped,
    Unreachable,
    Exited,
    Retrying,
}

pub struct SshConfig {
    end_host: String,
    username: String,
    key_path: String,
    to_host: String,
    local_port: u32,
    remote_port: u32,
    keepalive: u32,
}

impl SshConfig {
    pub fn new(
        end_host: &str,
        username: &str,
        key_path: &str,
        to_host: &str,
        local_port: u32,
        remote_port: u32,
        keepalive: u32,
    ) -> Self {
        SshConfig {
            end_host: String::from(end_host),
            username: String::from(username),
            key_path: String::from(key_path),
            to_host: String::from(to_host),
            local_port,
            remote_port,
            keepalive,
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        vec![
            String::from("-T"),
            String::from("-o"),
            String::from("UserKnownHostsFile=/dev/null"),
            String::from("-o"),
            String::from("StrictHostKeyChecking=no"),
            String::from("-o"),
            String::from("ServerAliveCountMax=1"),
            String::from("-o"),
            format!("ServerAliveInterval={}", self.keepalive),
            String::from("-L"),
            format!("{}:{}:{}", self.local_port, self.to_host, self.remote_port),
            String::from("-i"),
            self.key_path.clone(),
            format!("{}@{}", self.username, self.end_host),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::SshConfig;

    #[test]
    fn test_config() {
        let config = SshConfig::new("endhost", "username", "keypath", "tohost", 1, 2, 10);

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
