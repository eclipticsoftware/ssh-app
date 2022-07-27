use std::io::Read;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use num_traits::FromPrimitive;

use crate::config::SshConfig;
use crate::status::{ExitCondition, Result, SshStatus};

/// Defines the necessary interface that a child process type must support to be used by the tunnel library
pub trait ChildProc {
    fn new(config: SshConfig) -> Result<SshTunnel<Self>>;

    /// Retrieves the stdout from the child
    fn stdout(&mut self) -> Result<process::ChildStdout>;

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

/// Wraps the standard process::Child struct
pub struct TunnelChild {
    child: process::Child,
}

impl ChildProc for TunnelChild {
    #[cfg(not(target_os = "windows"))]
    fn new(config: SshConfig) -> Result<SshTunnel<Self>> {
        log::debug!("Starting ssh process");
        let child = process::Command::new("ssh")
            .args(config.to_args())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .map_err(|err| SshStatus::AppError(err.to_string()))?;

        log::debug!("New child address: {:p}", &child);
        Ok(Arc::new(Mutex::new(TunnelChild { child })))
    }

    #[cfg(target_os = "windows")]
    fn new(config: SshConfig) -> Result<SshTunnel<Self>> {
        let mut cmd = process::Command::new("ssh");

        for arg in config.to_args() {
            cmd.raw_arg(arg);
        }

        log::debug!("Starting ssh process");
        let child = cmd
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .map_err(|err| SshStatus::AppError(err.to_string()))?;

        log::debug!("New child address: {:p}", &child);
        Ok(Arc::new(Mutex::new(TunnelChild { child })))
    }

    fn stdout(&mut self) -> Result<process::ChildStdout> {
        log::debug!("Getting stdout from {:p}", &self.child);
        if let Some(stdout) = self.child.stdout.take() {
            Ok(stdout)
        } else {
            Err(SshStatus::AppError(
                "Failed to capture stdout of ssh process".to_string(),
            ))
        }
    }

    fn exited(&mut self) -> Option<ExitCondition> {
        match self.child.try_wait() {
            Ok(Some(status)) => {
                log::debug!("{:p} Exited with {status}", &self.child);
                if let Some(code) = status.code() {
                    match FromPrimitive::from_i32(code) {
                        Some(cond) => Some(cond),
                        None => {
                            log::warn!("Unknown exit code: {code}");
                            Some(ExitCondition::ProcError)
                        }
                    }
                } else {
                    Some(ExitCondition::Canceled)
                }
            }
            Ok(None) => None,
            Err(e) => {
                log::error!("Error attempting to wait for {:p}: {e}", &self.child);
                Some(ExitCondition::ProcError)
            }
        }
    }

    //Capture stderr to discover exit reason
    fn exit_status(&mut self) -> SshStatus {
        log::debug!("Getting exit status from {:p}", &self.child);
        let mut stderr = if let Some(stderr) = self.child.stderr.take() {
            stderr
        } else {
            return SshStatus::AppError("Failed to capture stderr of ssh process".to_string());
        };

        let mut stderr_msg = String::new();
        if stderr.read_to_string(&mut stderr_msg).is_err() {
            SshStatus::AppError("Failed to read from stderr".to_string())
        } else {
            let stderr_msg: String = stderr_msg
                .split('\n')
                .filter(|s| !s.contains("Warning: Permanently added") && !s.is_empty())
                .collect();
            log::info!("exit status: {stderr_msg}");
            SshStatus::from_stderr(&stderr_msg)
        }
    }

    fn kill(&mut self) {
        log::debug!("Killing {:p}", &self.child);
        match self.child.kill() {
            Ok(_) => {
                log::debug!("killed");
            }
            Err(err) => {
                log::debug!("Not killed: {err}")
            }
        }
    }
}
