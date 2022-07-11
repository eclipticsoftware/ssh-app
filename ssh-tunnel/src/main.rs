use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use std::io::Read;

use clap::{Parser};

use ssh_tunnel::{self, start_ssh_tunnel};
use ssh_tunnel::SshConfig;

fn main() -> Result<(), i32> {

    let args = Args::parse();
    let config = args.to_config();
    let tun_proc = start_ssh_tunnel(config).map_err(
        |err| {
            println!("Failed to create tunnel: {:?}", err);
            1
        }
    )?;

    let proc_hdlr = tun_proc.clone();
    ctrlc::set_handler(move || {
        println!("\n\nClosing tunnel");
        let mut tun_proc = proc_hdlr.lock().unwrap();
        match tun_proc.kill() {
            Ok(_) => {
                println!("killed");
            },
            Err(err) => {println!("Not killed: {err}")}
        }

        //std::process::exit(0);
    }).map_err(
        |err| {
            println!("Failed to set handler: {:?}", err);
            1
        }
    )?;


    let mut stderr;
    {
        stderr = tun_proc.lock().unwrap().stderr.take().unwrap();
    }
    

    let mut exit_status = 0;
    loop {
        let mut tun_proc = tun_proc.lock().unwrap();
        match tun_proc.try_wait() {
            Ok(Some(status)) => {
                println!("exited with {status}");
                exit_status = status.code().unwrap();
                break;
            },
            Ok(None) => (),
            Err(e) => {
                println!("error attempting to wait: {e}");
                exit_status = 100;
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    let mut error_msg = String::new();
    stderr.read_to_string(&mut error_msg).unwrap();

    println!("Error message: {error_msg}");

    Err(exit_status)
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
