use num_derive::FromPrimitive;
use regex::Regex;
use std::fmt;

/// Defines the set of statuses that ssh tunnel can have
#[derive(Debug, Clone, PartialEq)]
pub enum SshStatus {
    /// The tunnel is disconnected. It has either never connected, or it has disconnected cleanly
    Ready,

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

    /// An unknown ssh error
    Unknown(String),

    /// There was an error with the configuration
    ConfigError(String),

    /// There was an error with the process
    AppError(String),
}

fn stderr_is_dropped(msg: &str) -> bool {
    let re = Regex::new("Timeout, server .* not responding")
        .expect("This should not happen: invalid regex expression");

    re.is_match(msg) || msg.contains("Connection reset")
}

fn stderr_is_unreachable(msg: &str) -> bool {
    msg.contains("timed out")
        || msg.contains("Network is unreachable")
        || msg.contains("Unknown error")
        || msg.contains("Could not resolve hostname")
}

impl SshStatus {
    /// Parses the stderr captured during the ssh process and parses it into an SshStatus
    pub fn from_stderr(msg: &str) -> Self {
        if msg.is_empty() {
            SshStatus::Ready
        } else if stderr_is_dropped(msg) {
            SshStatus::Dropped
        } else if stderr_is_unreachable(msg) {
            SshStatus::Unreachable
        } else if msg.contains("Permission denied") || msg.contains("Connection refused") {
            SshStatus::Denied
        } else if msg.contains("Bad local forwarding specification") {
            SshStatus::ConfigError(msg.to_string())
        } else {
            log::debug!("Other status: {msg}");
            SshStatus::Unknown(msg.to_string())
        }
    }

    /// Converts the status to a "signal" string for status event signaling
    pub fn to_signal(&self) -> String {
        match self {
            SshStatus::Ready => "READY".to_string(),
            SshStatus::Connecting => "CONNECTING".to_string(),
            SshStatus::Connected => "CONNECTED".to_string(),
            SshStatus::Unreachable => "UNREACHABLE".to_string(),
            SshStatus::Denied => "DENIED".to_string(),
            SshStatus::Dropped => "DROPPED".to_string(),
            SshStatus::Reconnecting => "RETRYING".to_string(),
            SshStatus::Unknown(msg) => {
                log::error!("Unknown error: {msg}");
                format!("UNKNOWN: {msg}")
            }
            SshStatus::ConfigError(msg) => {
                log::error!("Config error: {msg}");
                format!("BAD_CONFIG: {msg}")
            }
            SshStatus::AppError(msg) => {
                log::error!("App error: {msg}");
                format!("ERROR: {msg}")
            }
        }
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
