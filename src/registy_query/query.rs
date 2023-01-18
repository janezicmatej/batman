use std::ops::Not;

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
    let project_url = info
        .get("project_url")
        .context("no project_url in info")?
        .as_str()
        .context("expected info.project_url to be string")?
        .to_string();

    // nested values
    let releases = json.get("releases").context("no releases in response")?;
    let _latest_release = releases.get(&version).context("no release in releases")?;

    let metadata = [
        ("name", name),
        ("author", author),
        ("version", version),
        ("url", project_url),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.is_empty().not().then_some(v)))
    .collect();
    let urls = info
        .get("project_urls")
        .context("no project_urls in info")?
        .as_object()
        .context("expected info.project_urls to be object")?
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.as_str().map(|x| x.to_string())))
        .collect();

    Ok(Package { metadata, urls })
}
