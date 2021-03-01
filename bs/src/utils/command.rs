use std::{ffi::OsStr, process::Command as StdCommand};

pub struct CommandBuilder {
    inner: StdCommand,
}

impl CommandBuilder {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        CommandBuilder {
            inner: StdCommand::new(program),
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.inner.arg(arg);
        self
    }

    pub fn env<S: AsRef<OsStr>>(&mut self, key: S, val: S) -> &mut Self {
        self.inner.env(key, val);
        self
    }

    fn args(&mut self, args: Vec<&OsStr>) -> &mut Self {
        self.inner.args(args);
        self
    }

    fn envs(&mut self, vars: Vec<(&OsStr, &OsStr)>) -> &mut Self {
        self.inner.envs(vars);
        self
    }

    fn get_program(&self) -> &OsStr {
        self.inner.get_program()
    }

    fn get_args(&self) -> Vec<&OsStr> {
        let mut args = Vec::new();

        for arg in self.inner.get_args().into_iter() {
            args.push(arg);
        }

        args
    }

    fn get_envs(&self) -> Vec<(&OsStr, &OsStr)> {
        let mut envs = Vec::new();
        for (env_var, env_val) in self.inner.get_envs().into_iter() {
            if let Some(s) = env_val {
                envs.push((env_var, s));
            }
        }

        envs
    }

    pub fn wrap(&self, other: Self) -> Self {
        let other_string: String = other.into();

        self.clone()
            .arg(&OsStr::new(&format!("'{}'", other_string)))
            .to_owned()
    }
}

impl Clone for CommandBuilder {
    fn clone(&self) -> Self {
        CommandBuilder::new(self.get_program())
            .args(self.get_args())
            .envs(self.get_envs())
            .to_owned()
    }
}

impl From<CommandBuilder> for String {
    fn from(cmd: CommandBuilder) -> Self {
        format!("{:?}", cmd.inner)
    }
}

impl From<CommandBuilder> for StdCommand {
    fn from(cmd: CommandBuilder) -> Self {
        cmd.inner
    }
}
