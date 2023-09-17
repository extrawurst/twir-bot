use chrono::{DateTime, Utc};
use model::Post;

mod model;

pub struct LobstersEntry {
    pub time: DateTime<Utc>,
    pub url: String,
}

pub async fn scrape_lobsters(until: DateTime<Utc>) -> Vec<LobstersEntry> {
    let newest: Vec<Post> = reqwest::Client::new()
        .get("https://lobste.rs/t/rust.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    newest
        .into_iter()
        .filter_map(|post| {
            if post.created_at > until {
                Some(LobstersEntry {
                    url: post.url,
                    time: post.created_at,
                })
            } else {
                None
            }
        })
        .collect()
}
