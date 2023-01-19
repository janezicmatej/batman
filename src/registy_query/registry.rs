use anyhow::Result;
use clap::ValueEnum;

use super::{package::Package, query::{query_pypi, query_npm}};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Registry {
    /// Python Packaged Index
    Pypi,
    /// Node Package Manager
    Npm,
    /// The Rust community's crate registry
    Cratesio,
}

impl Registry {
    pub async fn query(&self, package: &str) -> Result<Package> {
        match self {
            Self::Cratesio => todo!(),
            Self::Npm => query_npm(package).await,
            Self::Pypi => query_pypi(package).await,
        }
    }
}
