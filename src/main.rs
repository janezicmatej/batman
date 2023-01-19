mod args;
mod registy_query;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use args::{Args, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.subcommand {
        Commands::Registry { registry, package } => {
            let package = registry.query(&package).await?;
            println!("{package}");
            Ok(())
        }
        Commands::Generate { shell } => {
            generate(
                shell,
                &mut Args::command(),
                "batman",
                &mut std::io::stdout(),
            );
            Ok(())
        }
    }
}
