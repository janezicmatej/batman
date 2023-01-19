use clap::{Parser, Subcommand};
use clap_complete::shells::Shell;

use crate::registy_query::Registry;

#[derive(Parser)]
#[command(author = "Matej Janežič <matej.janezic@aflabs.si>")]
#[command(version)]
#[command(about = "Collection of tools for developers", long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate completion files
    Generate {
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Search for package in various package registries
    #[clap(alias = "rl")]
    Registry {
        /// Registry to query against
        #[arg(value_enum)]
        registry: Registry,

        /// Package to query against registry
        #[arg(value_enum)]
        package: String,
    },

    /// Ssh to a configured remote
    ///
    /// Glorified ssh aliases. You can configure remotes in
    /// $HOME/.batman/ssh_config.yaml
    #[clap(alias = "s")]
    Ssh {
        /// Remote name
        #[arg(value_enum)]
        remote: String,
    },
}
