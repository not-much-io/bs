use anyhow::{Error, Result};

pub mod config;
pub mod impls;

pub type SyncResult = Result<(), Error>;

pub trait StorageSyncer {
    fn sync(&self) -> SyncResult;
}
