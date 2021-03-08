use std::process::Command;

use anyhow::{anyhow, Result};

#[derive(Clone)]
pub enum Environment {
    Local,
    LocalDocker {
        dockerfile:     String,
        image_name:     String,
        container_name: String,
    },
    RemoteSsh,
    RemoteDocker,
}

impl Environment {
    pub fn new_local() -> Self {
        Environment::Local
    }

    pub fn new_local_docker(id: String, dockerfile: String) -> Self {
        let image_name = format!("{}-image", id);
        let container_name: String = format!("{}-container", id);

        Environment::LocalDocker {
            dockerfile,
            image_name,
            container_name,
        }
    }

    pub fn setup(&self) -> Result<()> {
        match self {
            Environment::Local => Ok(()),
            Environment::LocalDocker {
                dockerfile,
                image_name,
                container_name,
            } => {
                println!("building docker image...");
                Command::new("docker")
                    .arg("build")
                    .arg("--file")
                    .arg(dockerfile)
                    .arg("--tag")
                    .arg(image_name)
                    .arg(".")
                    .output()
                    .map_err(|err| anyhow!("Failed to build docker image: {}", err))?;

                println!("creating docker container...");
                Command::new("docker")
                    .arg("create")
                    .arg("--name")
                    .arg(container_name)
                    .arg("volume")
                    .arg("${PWD}:/src")
                    .arg(image_name)
                    .output()
                    .map_err(|err| anyhow!("Failed to create docker container: {}", err))?;

                println!("starting docker container...");
                Command::new("docker")
                    .arg("start")
                    .arg(container_name)
                    .output()
                    .map_err(|err| anyhow!("Failed to start docker container: {}", err))?;

                Ok(())
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
