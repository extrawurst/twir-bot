use chrono::{DateTime, Utc};
use serde::Deserialize;

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
    pub submitter_user: User,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub is_admin: bool,
    pub about: String,
    pub is_moderator: bool,
    pub karma: Option<i64>,
    pub avatar_url: String,
    pub invited_by_user: String,
    pub github_username: Option<String>,
}
