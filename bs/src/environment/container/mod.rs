pub mod docker;

use crate::utils::command::CommandBuilder;

use self::docker::Docker;

pub enum Container {
    DockerContainer(Docker),
}

impl Container {
    pub fn setup(&self) {
        match self {
            Container::DockerContainer(docker) => {
                docker.setup();
            }
        }
    }

    pub fn cmd(self, cmd: CommandBuilder) {
        match self {
            Container::DockerContainer(mut docker) => {
                docker.cmd(cmd);
                // &mut self
            }
        }
    }
}
