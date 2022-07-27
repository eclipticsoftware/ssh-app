use num_derive::FromPrimitive;
use regex::Regex;
use std::fmt;
use std::result;

/// Standard [std::result::Result] type for most ssh-tunnel functions.
///
/// We're following the [std::io::Result] practice here, which trades specificity for readability. Since the vast majority of
/// the functions in this library return [result::Result<T, SshStatus>] values, this seems like a valid trade-off.
pub type Result<T> = result::Result<T, SshStatus>;

/// Defines the set of statuses that ssh tunnel can have
///
/// Each state is considered either a **Success** state (if the system is working properly), an **Error** state if it's not,
/// or a **Transition** state, if it's in the process of changing states (essentially, while it's waiting to connect). These
/// categories are merely for conceptual purposes, they are not enforced in any way.
///
/// The transition states are not used internally in the library, but are provided as utility states for client applications.
#[derive(Debug, Clone, PartialEq)]
pub enum SshStatus {
    /// The tunnel is ready to connect. It has either never connected, or it has disconnected cleanly.
    ///
    /// This is a **Success** state
    Ready,

    /// The tunnel is connecting
    ///
    /// This is a **Transition** state
    Connecting,

    /// The tunnel is connected
    ///
    /// This is a **Success** state
    Connected,

    /// The server is unreachable
    ///
    /// This is an **Error** state
    Unreachable,

    /// The server has denied access
    ///
    /// This is an **Error** state
    Denied,

    /// The tunnel has dropped
    ///
    /// This is an **Error** state
    Dropped,

    /// The tunnel is trying to reconnect
    ///
    /// This is a **Transition** state
    Reconnecting,

    /// An unknown ssh error
    ///
    /// This is an **Error** state
    Unknown(String),

    /// There was an error with the configuration
    ///
    /// This is an **Error** state
    ConfigError(String),

    /// There was an error with the process
    ///
    /// This is an **Error** state
    AppError(String),
}

/// Checks whether the stderr message means that the connection has been dropped
fn stderr_is_dropped(msg: &str) -> bool {
    let re = Regex::new("Timeout, server .* not responding")
        .expect("This should not happen: invalid regex expression");

    re.is_match(msg) || msg.contains("Connection reset")
}

/// Checks whether the stderr message means that the server is unreachable
fn stderr_is_unreachable(msg: &str) -> bool {
    msg.contains("timed out")
        || msg.contains("Network is unreachable")
        || msg.contains("Unknown error")  // This error message sucks to handle here, but here we are
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
    ///
    /// This is not used internally in the library, but it provides a standard set of signals for client applications to use.
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

// Implement Display to make it easy to format the SshStatus to a string.
impl fmt::Display for SshStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Defines the set of exit conditions for the tunnel process
///
/// These are minimally useful. In most cases, the [SshStatus], parsed from the child's stderr will provide all of the
/// necessary information.
#[derive(FromPrimitive, Debug)]
pub enum ExitCondition {
    /// The tunnel exited cleanly. This actually will only happen if something
    /// goes wrong. A successful tunnel must be killed, which will result in
    /// an SshError or a Canceled condition.
    Clean = 0,

    /// An error occurred while handling the tunnel's child process
    ProcError = 1,

    /// The code that the ssh command will return if any error is encountered
    SshError = 255,

    /// No exit code was returned, probably because the process was canceled
    Canceled = -1,
}
