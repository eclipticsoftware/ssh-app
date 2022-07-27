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
                "StrictHostKeyChecking=accept-new",
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
        log::debug!("Args: {:?}", args);
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
