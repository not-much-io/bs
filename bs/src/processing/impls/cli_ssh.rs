use std::process::Command;

use crate::processing::{config::ProcessingConfig, ExecutorResult, ProcessingExecutor};

pub struct CliSsh {
    config: ProcessingConfig,
}

impl CliSsh {
    pub fn new(config: ProcessingConfig) -> Self {
        CliSsh { config }
    }
}

impl ProcessingExecutor for CliSsh {
    fn execute(&self) -> ExecutorResult {
        Command::new("ssh")
            .arg(format!(
                "{}@{}",
                self.config.server_user, self.config.server_ip
            ))
            .arg(self.config.commands.join(" && "));
        Ok(())
    }
}
