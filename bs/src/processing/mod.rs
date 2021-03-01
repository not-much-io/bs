use anyhow::{Error, Result};

pub mod config;
pub mod impls;

pub type ExecutorResult = Result<(), Error>;

pub trait ProcessingExecutor {
    fn execute(&self) -> ExecutorResult;
}
