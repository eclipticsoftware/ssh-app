/// Configuration parameters for the ssh tunnel
///
/// This struct provides all of the parameters necessary for launching an ssh tunnel.
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

    /// The keepalive time (in seconds). If the connection is interrupted for longer than this interval, the tunnel process
    /// will exit
    keepalive: u32,

    /// Any additional flags required
    flags: Vec<String>,
}

impl SshConfig {
    /// Construct an SshConfig object
    ///
    /// This function parameters provide all of those necessary for launching an ssh tunnel. Any extra parameters that are
    /// needed can be passed in the flags parameter.
    ///
    /// # Params
    /// * `end_host`: The address of the end (or remote) host.
    ///
    /// * `username`: The username for the remote host.
    ///
    /// * `key_path`: The path to the private key to use.
    ///
    /// * `to_host`:  The address of the local host (probably "localhost").
    ///
    /// * `local_port`: The local port to use.
    ///
    /// * `remote_port`: The remote port to use.
    ///
    /// * `keepalive`: The keepalive time (in seconds). If the connection is interrupted for longer than this interval, the
    ///    tunnel process.
    ///
    /// * `flags`: A vector of additional flags or options to pass to the process cli.
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
        // Ensure that the path conforms to the unix-y paths that ssh prefers
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

    /// Converts the config object to an argument vector useful for passing to the ssh cli.
    ///
    /// This provides all of the arguments necessary for creating an ssh tunnel connection. Additional arguments provided by
    /// the `flags` parameter to [SshConfig::new] are prepended to the front of the default arguments. Here's a brief
    /// explanation of each of the default arguments (see the [ssh(1) man](https://linux.die.net/man/1/ssh) and
    /// [ssh_config(5) man ](https://linux.die.net/man/5/ssh_config) pages):
    ///
    /// * **-o StrictHostKeyChecking=accept-new**: Automatically adds new host keys to the user known host file, but does not
    ///   permit connections to hosts with changed host keys. This setting allows the app to connect without needing to
    ///   a query on whether to add a new host, but also keeps the security risk from man-in-the-middle attacks low.
    ///
    /// * **-o ServerAliveCountMax=1**: Instructs the tunnel to shut down after the first alive message is missed.
    ///
    /// * **-o ServerAliveInterval=<keepalive>**: Sets the interval for the alive message. This defines the response time to a
    ///   server disconnect event.
    ///
    /// * **-L local_port:local_host:remote_port**: Forwards the local host & port to the remote port. This is the option that
    ///   makes this a tunnel.
    ///
    /// * **-i identity_file**: Path to the private key that will be used.
    ///
    /// * **user@host**: The username and host address for the remote host.
    pub fn to_args(&self) -> Vec<String> {
        let mut args = self.flags.clone();
        args.append(
            &mut vec![
                "-o",
                "StrictHostKeyChecking=accept-new",
                "-o",
                "ServerAliveInterval=1",
                "-o",
                &format!("ServerAliveCountMax={}", self.keepalive),
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
