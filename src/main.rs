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
        Commands::Ssh { remote } => match remote {
            Some(remote) => Host::for_remote(&remote)?.ssh_connect(),
            None => {
                let mut hosts = Host::load()?.into_iter().collect::<Vec<(String, _)>>();
                hosts.sort_by(|a, b| a.0.cmp(&b.0));
                let pad = hosts.iter().map(|x| x.0.len()).max().unwrap();

                for (name, host) in hosts {
                    println!("{name: <pad$} {host}");
                }
                Ok(())
            }
        },
        Commands::Dplo { name } => match name {
            Some(name) => Url::for_name(&name)?.open(),
            None => {
                let mut urls = Url::load()?.into_iter().collect::<Vec<(String, _)>>();
                urls.sort_by(|a, b| a.0.cmp(&b.0));
                let pad = urls.iter().map(|x| x.0.len()).max().unwrap();

                for (name, url) in urls {
                    println!("{name: <pad$} {url}");
                }
                Ok(())
            }
        },
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
