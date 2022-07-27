use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use clap::Parser;
use ssh_tunnel::{
    config::SshConfig,
    logger,
    status::{ExitCondition, SshStatus},
    tunnel::{ChildProc, SshHandle, SshTunnel, TunnelChild},
};

fn main() -> Result<(), i32> {
    let args = Args::parse();

    let mut logpath = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("."));

    logpath.push(".eclo-ssh-client.log");

    let logpath = logpath
        .into_os_string()
        .into_string()
        .expect("Failed to create log path");

    logger::configure_logger(&logpath).map_err(|err| {
        println!("Failed to configure logger: {err}");
        1
    })?;

    log::debug!("Running SSH Tunnel CLI");

    let config = args.to_config();

    let exit_callback = Arc::new(Mutex::new(|status| {
        log::info!("Status: {:?}", status);
        match status {
            SshStatus::Dropped => log::info!("Dropped connection"),
            SshStatus::Unreachable => log::warn!("Unreachable"),
            SshStatus::Ready => log::info!("Disconnected cleanly"),
            _ => log::error!("Unsupported status: {:?}", status),
        }
    }));

    let tunnel: SshTunnel<TunnelChild>;
    let handle: SshHandle;
    match ssh_tunnel::start_and_watch_ssh_tunnel(config, exit_callback, true) {
        Ok((tnl, hndl)) => {
            tunnel = tnl;
            handle = hndl;
        }
        Err(err) => {
            log::error!("Failed to create tunnel: {:?}", err);
            return Err(ExitCondition::SshError as i32);
        }
    }

    ctrlc::set_handler(move || {
        log::info!("Closing tunnel");
        let mut tunnel = tunnel.lock().unwrap();
        tunnel.kill();
    })
    .map_err(|err| {
        log::error!("Failed to set handler: {:?}", err);
        100
    })?;

    log::info!("SSH tunnel started");
    let (ssh_status, exit_status) = handle.join().unwrap();
    match ssh_status {
        SshStatus::Ready => Ok(()),
        _ => Err(exit_status as i32),
    }
}

#[derive(Parser)]
#[clap(version = "ssh-tunnel 0.1.0", long_about = None)]
#[clap(about = "Create ssh tunnel")]
struct Args {
    /// Endhost name
    end_host: String,

    /// Username
    username: String,

    /// Path to the key file to use
    key_path: String,

    /// Tohost name
    to_host: String,

    /// Local port number
    #[clap(short, long, default_value = "5432")]
    local_port: u32,

    /// Remote port number
    #[clap(short, long, default_value = "5432")]
    remote_port: u32,

    /// Keepalive time (in seconds)
    #[clap(short, long, default_value = "10")]
    keepalive: u32,
}

impl Args {
    fn to_config(&self) -> SshConfig {
        SshConfig::new(
            &self.end_host,
            &self.username,
            &self.key_path,
            &self.to_host,
            self.local_port,
            self.remote_port,
            self.keepalive,
            &["-T"],
        )
    }
}
