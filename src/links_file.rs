use anyhow::Result;
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

#[derive(Clone)]
pub struct LinksFile {
    current: Arc<RwLock<String>>,
}

impl LinksFile {
    pub async fn new() -> Result<Self> {
        let body = reqwest::get(
            "https://raw.githubusercontent.com/rust-lang/this-week-in-rust/master/links.txt",
        )
        .await?
        .text()
        .await?;

        let current = Arc::new(RwLock::new(body));

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
        let body = reqwest::get(
            "https://raw.githubusercontent.com/rust-lang/this-week-in-rust/master/links.txt",
        )
        .await?
        .text()
        .await?;

        tracing::info!("updated links file: {} bytes", body.len());

        let mut content = content.write().await;
        *content = body;

        Ok(())
    }
}
