use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Article {
    pub type_of: String,
    pub id: i64,
    pub title: String,
    pub description: String,
    pub readable_publish_date: String,
    pub slug: String,
    pub path: String,
    pub url: String,
    pub comments_count: i64,
    pub public_reactions_count: i64,
    pub collection_id: Option<i64>,
    pub published_timestamp: DateTime<Utc>,
    pub cover_image: Option<String>,
    pub social_image: String,
    pub canonical_url: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    // pub crossposted_at: String, // null,
    pub published_at: DateTime<Utc>,
    pub last_comment_at: Option<DateTime<Utc>>,
    pub reading_time_minutes: i64,
    pub tag_list: Vec<String>,
    pub tags: String,
    // pub user: User,
    // pub organization: Organization,
    // pub flare_tag; FlareTag,
}
