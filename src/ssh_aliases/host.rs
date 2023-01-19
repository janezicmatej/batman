use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

use crate::config::get_config_path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Host {
    user: String,
    host: String,
    port: Option<u32>,
}

impl Host {
    pub fn for_remote(remote: &str) -> Result<Self> {
        let hosts = std::fs::read_to_string(get_config_path("ssh_hosts.yaml")?)?;
        let mut parsed = serde_yaml::from_str::<HashMap<String, Host>>(&hosts)?;

        parsed
            .remove(remote)
            .context(format!("remote {remote} is not defined"))
    }

    pub fn ssh_connect(&self) -> Result<()> {
        let port = self.port.unwrap_or(22).to_string();
        let user_host = format!("{}@{}", &self.user, &self.host);
        let mut command = Command::new("ssh");
        command
            .args(["-t", "-p", &port, &user_host])
            .spawn()
            .context("unable to spawn ssh command")?
            .wait()
            .context("failed while waiting for command to execute")?;

        Ok(())
    }
}
