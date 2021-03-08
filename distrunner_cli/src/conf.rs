use std::{collections::HashMap, fs::File, path::PathBuf};

use distrunner::{environment::Environment, process::Process, DistRunner, RunUnit};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DistRunnerConfig {
    environments: Vec<EnvironmentConfig>,
    processes:    Vec<ProcessConfig>,
    run_plan:     Vec<RunPlanConfig>,
}

impl DistRunnerConfig {
    pub fn new(config_path: PathBuf) -> Self {
        serde_json::from_reader(File::open(config_path).unwrap()).unwrap()
    }
}

impl From<DistRunnerConfig> for DistRunner {
    fn from(config: DistRunnerConfig) -> Self {
        let mut env_registry = HashMap::new();
        for env_conf in config.environments {
            let id: EnvironmentID = env_conf.id.clone();
            let env: Environment = env_conf.into();

            env_registry.insert(id, env);
        }

        let mut proc_registry = HashMap::new();
        for proc_conf in config.processes {
            let id: ProcessID = proc_conf.id.clone();
            let proc: ProcessConfig = proc_conf;

            proc_registry.insert(id, proc);
        }

        let mut run_units = Vec::new();
        for run_plan in config.run_plan {
            let env = env_registry.remove(&run_plan.environment_id).unwrap();
            let proc = proc_registry
                .remove(&run_plan.process_id)
                .unwrap()
                .into_process(&env);

            run_units.push(RunUnit::new(run_plan.id, env, proc));
        }

        DistRunner::new(run_units)
    }
}

#[derive(Deserialize)]
enum EnvironmentType {
    Local,
    LocalDocker,
}

type EnvironmentID = String;

#[derive(Deserialize)]
struct EnvironmentConfig {
    id: EnvironmentID,

    #[serde(rename = "type")]
    type_:      EnvironmentType,
    dockerfile: Option<PathBuf>,
}

impl From<EnvironmentConfig> for Environment {
    fn from(config: EnvironmentConfig) -> Self {
        match config.type_ {
            EnvironmentType::Local => Environment::new_local(),
            EnvironmentType::LocalDocker => {
                let dockerfile_string = config
                    .dockerfile
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();
                Environment::new_local_docker(config.id, dockerfile_string)
            }
        }
    }
}

type ProcessID = String;

#[derive(Deserialize)]
struct ProcessConfig {
    id:  ProcessID,
    cmd: String,
}

impl ProcessConfig {
    fn into_process(self, env: &Environment) -> Process {
        match env {
            Environment::Local => Process::new_local(self.cmd),
            Environment::LocalDocker { container_name, .. } => {
                Process::new_local_docker(self.cmd, container_name.clone())
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

type RunUnitID = String;

#[derive(Deserialize)]
struct RunPlanConfig {
    id:             RunUnitID,
    environment_id: EnvironmentID,
    process_id:     ProcessID,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::DistRunnerConfig;

    #[test]
    fn test_local_deserialize() {
        let file = File::open("../its/projects/generic/distrunner.json").unwrap();
        let _config: DistRunnerConfig = serde_json::from_reader(file).unwrap();
    }
}
