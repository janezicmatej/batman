mod args;
mod config;
mod registy_query;
mod ssh_aliases;
mod url_opener;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use args::{Args, Commands};
use ssh_aliases::host::Host;
use url_opener::url::Url;

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
        Commands::Dplo { name } => Url::for_name(&name)?.open(),
        Commands::Generate { shell } => {
            generate(shell, &mut Args::command(), "bm", &mut std::io::stdout());
            Ok(())
        }
    }
}
