use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::get_config_path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Url {
    url: String,
}

impl Url {
    pub fn for_name(name: &str) -> Result<Self> {
        let hosts = std::fs::read_to_string(get_config_path("urls.yaml")?)?;
        let mut parsed = serde_yaml::from_str::<HashMap<String, Url>>(&hosts)?;

        parsed
            .remove(name)
            .context(format!("remote {name} is not defined"))
    }

    pub fn open(&self) -> Result<()> {
        open::that(&self.url)?;

        Ok(())
    }
}
