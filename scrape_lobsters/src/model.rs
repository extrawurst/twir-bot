use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Post {
    pub short_id: String,
    pub short_id_url: String,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub url: String,
    pub score: i64,
    pub flags: i64,
    pub comment_count: u64,
    pub description: String,
    pub description_plain: String,
    pub comments_url: String,
    pub submitter_user: String,
    pub tags: Vec<String>,
}
