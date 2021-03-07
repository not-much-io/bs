use std::process::{Child, Command, Stdio};

use anyhow::{anyhow, Result};

pub enum Process {
    LocalProcess { cmd: Command },
    LocalDockerProcesss {},
    RemoteProcess {},
    RemoteDockerProcess {},
}

impl Process {
    pub fn new_local(cmd: String) -> Self {
        let mut cmd_tokens = cmd.split(' ');
        let program = cmd_tokens.next().unwrap();
        let mut cmd = Command::new(program);
        cmd.stdout(Stdio::piped());

        for token in cmd_tokens {
            cmd.arg(token);
        }

        Process::LocalProcess { cmd }
    }

    pub async fn spawn(&mut self) -> Result<ProcessHandle> {
        match self {
            Process::LocalProcess { cmd } => {
                Ok(ProcessHandle::new_local(cmd.spawn().map_err(|err| {
                    anyhow!("Faield to spawn process: {}", err)
                })?))
            }
            Process::LocalDockerProcesss {} => {
                todo!()
            }
            Process::RemoteProcess {} => {
                todo!()
            }
            Process::RemoteDockerProcess {} => {
                todo!()
            }
        }
    }
}

pub enum ProcessHandle {
    LocalProcessHandle(Child),
}

impl ProcessHandle {
    fn new_local(child: Child) -> Self {
        ProcessHandle::LocalProcessHandle(child)
    }

    pub fn kill(&mut self) -> Result<()> {
        match self {
            ProcessHandle::LocalProcessHandle(handle) => handle
                .kill()
                .map_err(|err| anyhow!("Failed to kill local process: {}", err)),
        }
    }

    pub fn wait_with_output(self) -> Result<String> {
        match self {
            ProcessHandle::LocalProcessHandle(handle) => {
                let output = handle
                    .wait_with_output()
                    .map_err(|err| anyhow!("Error occurred in process: {}", err))?;

                Ok(String::from_utf8(output.stdout).unwrap())
            }
        }
    }
}
