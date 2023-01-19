use std::ops::Not;

use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde_json::Value;

use super::package::Package;

trait Seperate {
    fn seperate(self) -> Self;
}

impl Seperate for String {
    fn seperate(self) -> Self {
        self.as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }
}

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

    let response_release = client
        .get(format!("https://pypi.org/pypi/{package}/{version}/json"))
        .send()
        .await?
        .text()
        .await?;

    let release_json: Value = serde_json::from_str(&response_release)?;

    // last release data
    let release_info = release_json.get("urls").context("no urls in response")?;

    let upload_time = Utc
        .datetime_from_str(
            release_info
                .as_array()
                .context("expected urls to be array")?[0]
                .get("upload_time")
                .context("no upload_time in urls.0")?
                .as_str()
                .context("expected upload_time to be string")?,
            "%Y-%m-%dT%H:%M:%S",
        )
        .context("unable to parse upload_time")?
        .format("%-d. %b %y")
        .to_string();

    // dowloads data
    let response_stats = client
        .get(format!(
            "https://pypistats.org/api/packages/{package}/recent?period=month"
        ))
        .send()
        .await?
        .text()
        .await?;

    let stats_json: Value = serde_json::from_str(&response_stats)?;

    let downloads = stats_json
        .get("data")
        .context("no data in response")?
        .get("last_month")
        .context("expected last_month in data")?
        .as_u64()
        .context("expected data.last_month to be a number")?
        .to_string()
        .seperate();

    let metadata = [
        ("name", name),
        ("author", author),
        ("version", version),
        ("uploaded", upload_time),
        ("downloads (month)", downloads),
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

    Ok(Package {
        groups: vec![metadata, urls],
    })
}

pub async fn query_npm(package: &str) -> Result<Package> {
    let client = Client::new();
    let response = client
        .get(format!("https://registry.npmjs.org/{package}"))
        .send();

    let response_stats = client
        .get(format!(
            "https://api.npmjs.org/downloads/point/last-month/{package}"
        ))
        .send();

    let json: Value = serde_json::from_str(&response.await?.text().await?)?;

    let name = json
        .get("name")
        .context("no name in response")?
        .as_str()
        .context("expected name to be string")?
        .to_string();
    let version = json
        .get("dist-tags")
        .context("no dist-tags in response")?
        .get("latest")
        .context("no latest in dist-tags")?
        .as_str()
        .context("expected dist-tags.latest to be string")?
        .to_string();
    let homepage = json
        .get("homepage")
        .context("no homepage in response")?
        .as_str()
        .context("expected homepage to be string")?
        .to_string();
    let upload_time = DateTime::parse_from_rfc3339(
        json.get("time")
            .context("no time in response")?
            .get(&version)
            .context("no latest version in time")?
            .as_str()
            .context("expected time.latest to be string")?,
    )
    .context("unable to parse upload_time")?
    .format("%-d. %b %y")
    .to_string();

    let stats_json: Value = serde_json::from_str(&response_stats.await?.text().await?)?;

    let downloads = stats_json
        .get("downloads")
        .context("no data in response")?
        .as_u64()
        .context("expected data.last_month to be a number")?
        .to_string()
        .seperate();

    let metadata = [
        ("name", name),
        ("version", version),
        ("uploaded", upload_time),
        ("downloads (month)", downloads),
        ("homepage", homepage),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.is_empty().not().then_some(v)))
    .collect();

    Ok(Package {
        groups: vec![metadata],
    })
}
