use std::str::FromStr;

use anyhow::Result;

use super::{package::Package, query::query_pypi};

#[derive(Clone, Copy)]
pub enum Registry {
    PyPI,
    Npm,
    CratesIO,
}

impl FromStr for Registry {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pypi" => Ok(Self::PyPI),
            "npm" => Ok(Self::Npm),
            "cratesio" => Ok(Self::CratesIO),
            _ => Err("unsuporrted registry".to_string()),
        }
    }
}

impl Registry {
    pub async fn query(&self, package: &str) -> Result<Package> {
        match self {
            Self::CratesIO => todo!(),
            Self::Npm => todo!(),
            Self::PyPI => query_pypi(package).await,
        }
    }
}
