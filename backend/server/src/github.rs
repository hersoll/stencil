use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, RwLock};

pub(crate) static GITHUB_RELEASES: LazyLock<RwLock<Vec<GithubRelease>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct GithubRelease {
    pub(crate) tag_name: String,
    pub(crate) created_at: String,
    pub(crate) body: Option<String>,
}

pub(crate) async fn fetch_github_releases() -> Result<Vec<GithubRelease>> {
    if db::production_mode() {
        let url = "https://api.github.com/repos/hersoll/stencil/releases";
        let body: Vec<GithubRelease> = reqwest::Client::new()
            .get(url)
            .header("User-Agent", "hersoll/stencil")
            .send()
            .await?
            .json()
            .await?;

        Ok(body)
    } else {
        // Don't fetch in dev mode, mock
        Ok(vec![GithubRelease {
            tag_name: "v0.0.7".to_string(),
            created_at: "1997-02-01T18:49:04Z".to_string(),
            body: Some("Dev mode!".to_string()),
        }])
    }
}

pub(crate) fn store_github_releases(releases: Vec<GithubRelease>) -> Result<()> {
    let mut vec = GITHUB_RELEASES
        .write()
        .expect("Failed to write to GITHUB_RELEASES");
    *vec = releases;
    Ok(())
}
