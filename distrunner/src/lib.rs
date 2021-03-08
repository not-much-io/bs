#![feature(command_access)]

use anyhow::{anyhow, Error, Result};
use environment::Environment;
use futures::future;
use process::Process;
use tokio::task::JoinHandle;

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
        for mut run_unit in self.run_units {
            let handle: JoinHandle<Result<_, Error>> = tokio::task::spawn(async move {
                run_unit
                    .environment
                    .setup()
                    .map_err(|err| anyhow!("Environment setup failed: {}", err))?;
                let proc_handle = run_unit
                    .process
                    .spawn()
                    .await
                    .map_err(|err| anyhow!("Process spawn failed {}:", err))?;

                Ok((run_unit.id, proc_handle.wait_with_output()?))
            });
            task_handles.push(handle);
        }

        for join_result in future::join_all(task_handles).await.iter() {
            let process_result = join_result.as_ref().unwrap();
            match process_result {
                Ok((id, output)) => {
                    println!("{} output:\n{}", id, output)
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
