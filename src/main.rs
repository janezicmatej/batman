mod args;
mod config;
mod registy_query;
mod ssh_aliases;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use args::{Args, Commands};
use ssh_aliases::host::Host;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.subcommand {
        Commands::Registry { registry, package } => {
            let package = registry.query(&package).await?;
            println!("{package}");
            Ok(())
        }
        Commands::Ssh { remote } => Host::for_remote(&remote)?.ssh_connect(),

        Commands::Generate { shell } => {
            generate(shell, &mut Args::command(), "bm", &mut std::io::stdout());
            Ok(())
        }
    }
}
