pub mod ssh;

use self::ssh::Ssh;

pub enum Remote {
    SshRemote(Ssh),
}

impl Remote {
    pub fn test(&self) {
        match self {
            Remote::SshRemote(ssh) => {
                ssh.test();
            }
        }
    }

    pub fn run_on(&self) {
        match self {
            Remote::SshRemote(ssh) => {
                ssh.run_on();
            }
        }
    }
}
