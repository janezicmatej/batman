use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use crate::config::get_config_path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Url {
    url: String,
}

impl Url {
    pub fn for_name(name: &str) -> Result<Self> {
        Self::load()?
            .remove(name)
            .context(format!("remote {name} is not defined"))
    }

    pub fn load() -> Result<HashMap<String, Url>> {
        let hosts = std::fs::read_to_string(get_config_path("urls.yaml")?)?;
        serde_yaml::from_str::<HashMap<String, Url>>(&hosts).context("unable to parse yaml")
    }

    pub fn open(&self) -> Result<()> {
        open::that(&self.url)?;

        Ok(())
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
