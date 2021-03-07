use anyhow::Result;

use crate::process::{Process, ProcessHandle};

#[derive(Clone)]
pub enum Environment {
    Local,
    LocalDocker,
    RemoteSsh,
    RemoteDocker,
}

impl Environment {
    pub fn new_local() -> Self {
        Environment::Local
    }

    pub fn setup(&self) {
        match self {
            Environment::Local => {
                // NOOP since nothing to set up
            }
            Environment::LocalDocker => {
                todo!()
            }
            Environment::RemoteSsh => {
                todo!()
            }
            Environment::RemoteDocker => {
                todo!()
            }
        }
    }
}
