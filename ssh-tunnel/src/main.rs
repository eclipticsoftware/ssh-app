use clap::Parser;

use ssh_tunnel::{SshConfig, SshStatus, ExitStatus};

fn main() -> Result<(), i32> {

    let args = Args::parse();
    let config = args.to_config();
    let (tun_proc, hndl) = ssh_tunnel::start_and_watch_ssh_tunnel(config, move |status| {
        println!("Status: {:?}", status);
        match status {
            SshStatus::Dropped => println!("Dropped connection"),
            SshStatus::Unreachable => println!("Unreachable"),
            SshStatus::Exited => println!("Exited cleanly"),
            _ => println!("Unsupported status: {:?}", status)
        }
    }).map_err(
        |err| {
            println!("Failed to create tunnel: {:?}", err);
            ExitStatus::SshError as i32
        }
    )?;

    ctrlc::set_handler(move || {
        println!("\n\nClosing tunnel");
        let mut tun_proc = tun_proc.lock().unwrap();
        match tun_proc.kill() {
            Ok(_) => {
                println!("killed");
            },
            Err(err) => {println!("Not killed: {err}")}
        }

    }).map_err(
        |err| {
            println!("Failed to set handler: {:?}", err);
            100
        }
    )?;

    println!("SSH tunnel started");
    let (ssh_status, exit_status) = hndl.join().unwrap();
    match ssh_status {
        SshStatus::Exited => Ok(()),
        _ => Err(exit_status as i32)
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
            self.keepalive
        )
    }
    
    
}
