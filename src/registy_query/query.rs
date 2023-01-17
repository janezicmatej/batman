use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

use super::package::Package;

pub async fn query_pypi(package: &str) -> Result<Package> {
    let client = Client::new();
    let response = client
        .get(format!("https://pypi.org/pypi/{package}/json"))
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&response)?;

    let info = json.get("info").context("no info in response")?;

    // retrieve available data for package from info
    let name = info
        .get("name")
        .context("no name in info")?
        .as_str()
        .context("expected info.name to be string")?
        .to_string();
    let version = info
        .get("version")
        .context("no version in info")?
        .as_str()
        .context("expected info.version to be string")?
        .to_string();
    let author = info
        .get("author")
        .context("no author in info")?
        .as_str()
        .context("expected info.author to be string")?
        .to_string();
    let url = info
        .get("project_url")
        .context("no project_url in info")?
        .as_str()
        .context("expected info.project_url to be string")?
        .to_string();

    // nested values
    let project_urls = info
        .get("project_urls")
        .context("no project_urls in info")?;
    let releases = json.get("releases").context("no releases in response")?;
    let latest_release = releases.get(&version).context("no release in releases")?;

    // rest
    let docs = project_urls
        .get("Documentation")
        .and_then(|v| Some(v.as_str()?.to_string()));
    let source = project_urls
        .get("Source")
        .and_then(|v| Some(v.as_str()?.to_string()));

    // let latest_release = json.get("releases").context("there is no releases")?.get(info.get("version"))

    Ok(Package {
        name,
        version,
        published: "xd".to_string(),
        author,
        url,
        docs,
        source,
    })
}
