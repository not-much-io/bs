#![feature(command_access)]

use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use environment::Environment;
use futures::future;
use process::Process;
use tokio::sync::Mutex;

pub mod environment;
pub mod network;
pub mod process;
pub mod storage;

pub struct DistRunner {
    run_units: Vec<RunUnit>,
}

impl DistRunner {
    pub fn new(run_units: Vec<RunUnit>) -> Self {
        DistRunner { run_units }
    }

    pub async fn run(self) -> Result<()> {
        let mut task_handles = Vec::new();
        for mut unit in self.run_units {
            let handle = tokio::task::spawn(async move {
                unit.environment.setup();
                let spawn_result = unit.process.spawn().await;

                match spawn_result {
                    Err(err) => Err(err),
                    Ok(env_handle) => env_handle.wait_with_output(),
                }
            });
            task_handles.push(handle);
        }

        for join_result in future::join_all(task_handles).await.iter() {
            let process_result = join_result.as_ref().unwrap();
            match process_result {
                Ok(output) => {
                    println!("output:\n{}", output)
                }
                Err(err) => {
                    println!("error:\n{}", err)
                }
            }
        }

        Ok(())
    }
}

pub type RunUnitID = String;

pub struct RunUnit {
    id:          RunUnitID,
    environment: Environment,
    process:     Process,
}

impl RunUnit {
    pub fn new(id: RunUnitID, environment: Environment, process: Process) -> Self {
        RunUnit {
            id,
            environment,
            process,
        }
    }
}
