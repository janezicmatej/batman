use std::path::PathBuf;

use anyhow::{Context, Result};
use home::home_dir;

pub fn get_config_path(config_name: &str) -> Result<PathBuf> {
    let config_dir = home_dir()
        .context("unable to get home dir")?
        .join(".batman");

    if !config_dir.is_dir() {
        std::fs::create_dir(&config_dir).context(format!(
            "unable to create dir {}",
            config_dir.to_string_lossy()
        ))?;
    }

    let config_file = config_dir.join(config_name);

    if !config_file.is_file() {
        std::fs::File::create(&config_file).context(format!(
            "unable to create file {}",
            config_file.to_string_lossy()
        ))?;
    }

    Ok(config_file)
}
