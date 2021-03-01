use std::path::PathBuf;

use crate::utils::command::CommandBuilder;

pub struct Docker {
    cmd:    Option<CommandBuilder>,
    config: DockerConfig,
}

impl Docker {
    pub fn setup(&self) {}

    pub fn cmd(&mut self, cmd: CommandBuilder) {
        self.cmd = Some(
            CommandBuilder::new("docker")
                .arg("exec")
                .arg("-it")
                .wrap(cmd)
                .clone(),
        );
        // self
    }
}

pub struct DockerConfig {
    dockerfile: PathBuf,
}

impl DockerConfig {
    pub fn new(dockerfile: PathBuf) -> Self {
        DockerConfig { dockerfile }
    }
}
