use anyhow::Result;
use git2::Repository;
use lazy_static::lazy_static;
use regex::Regex;
use std::{path::Path, sync::Arc, time::Duration};
use tokio::{fs::read_dir, sync::RwLock, time::sleep};

lazy_static! {
    pub static ref MD_REGEX: Regex = Regex::new(r#"\*.*\[.*\]\((.*)\)"#).unwrap();

    // slighly modified:
    // https://www.geeksforgeeks.org/python-check-url-string/
    pub static ref URL_REGEX: Regex = Regex::new(
        r#"(?i)\b((?:https?://|www\d{0,3}[.]|[a-z0-9.\-]+[.][a-z]{2,4}/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()\[\]{};:'.,<>?«»“”‘’]))"#).unwrap();
}

#[derive(Clone)]
pub struct LinksFile {
    current: Arc<RwLock<String>>,
}

impl LinksFile {
    pub async fn new() -> Result<Self> {
        let links = Self::gather_links().await?;

        let current = Arc::new(RwLock::new(links));

        Self::update_loop(current.clone());

        Ok(Self { current })
    }

    pub async fn contains(&self, content: String) -> bool {
        let current = self.current.read().await;
        current.find(&content).is_some()
    }

    fn update_loop(content: Arc<RwLock<String>>) {
        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::update_file(content.clone()).await {
                    tracing::error!("update file error: {}", e);
                }

                sleep(Duration::from_secs(60 * 60 * 2)).await;
            }
        });
    }

    async fn update_file(content: Arc<RwLock<String>>) -> Result<()> {
        let links = Self::gather_links().await?;

        tracing::info!("updated links file: {} bytes", links.len());

        let mut content = content.write().await;
        *content = links;

        Ok(())
    }

    async fn gather_links() -> Result<String> {
        use tempfile::tempdir;

        let tmpdir = tempdir()?;
        {
            Repository::clone(
                "https://github.com/rust-lang/this-week-in-rust.git",
                &tmpdir,
            )?;
        }

        let mut entries: Vec<String> = Vec::new();
        let mut traversal = read_dir(tmpdir.into_path().join("content")).await?;
        while let Some(e) = traversal.next_entry().await? {
            if e.path().is_file()
                && e.path()
                    .extension()
                    .map(|ext| ext == "md")
                    .unwrap_or_default()
            {
                if let Ok(entries_in_file) = Self::parse_links(&e.path()).await {
                    // tracing::info!("{:?} [{}]", e.path(), entries_in_file.len());
                    entries.extend_from_slice(&entries_in_file);
                }
            }
        }

        tracing::info!("found entries: {:?}", entries.len());

        Ok(entries.join("\n"))
    }

    async fn parse_links(p: &Path) -> Result<Vec<String>> {
        let content = tokio::fs::read(p).await?;
        let content = String::from_utf8_lossy(&content);

        let res = content
            .lines()
            .filter_map(|line| MD_REGEX.captures(line).and_then(|r| r.get(1)))
            .map(|m| m.as_str().to_string())
            .collect::<Vec<_>>();

        Ok(res)
    }
}
