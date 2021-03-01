pub mod container;
pub mod remote;

use crate::utils::command::CommandBuilder;

use self::{container::Container, remote::Remote};

pub enum Environment {
    Local,
    LocalContainer(Container),
    Remote(Remote),
    RemoteContainer(Remote, Container),
}

impl Environment {
    pub fn setup(&self) {
        match self {
            Environment::Local => {}
            Environment::LocalContainer(container) => match container {
                Container::DockerContainer(docker) => {
                    docker.setup();
                }
            },
            Environment::Remote(remote) => match remote {
                Remote::SshRemote(_) => {}
            },
            Environment::RemoteContainer(_remote, _container) => {}
        }
    }

    pub fn run_on(&self, _cmd: CommandBuilder) {
        match self {
            Environment::Local => {}
            Environment::LocalContainer(_) => {}
            Environment::Remote(_) => {}
            Environment::RemoteContainer(_, _) => {}
        }
    }
}
