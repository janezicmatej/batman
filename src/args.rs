use clap::{Parser, Subcommand};
use clap_complete::shells::Shell;

use crate::registy_query::Registry;

#[derive(Parser)]
#[command(author = "Matej Janežič <matej.janezic@aflabs.si>")]
#[command(version)]
#[command(about = "Collection of tools for developers", long_about = None)]
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
    Registry {
        #[arg(value_enum)]
        registry: Registry,

        /// View package on registry
        #[arg(long, short, default_value_t = false)]
        open: bool,

        /// View package docs
        #[arg(long, short, default_value_t = false)]
        docs: bool,

        /// View package source
        #[arg(long, short, default_value_t = false)]
        source: bool,

        /// Package to query against registry
        #[arg(value_enum)]
        package: String
    },
}
