use chrono::{DateTime, Utc};
use roux::{util::FeedOption, Subreddit};

pub struct RedditEntry {
    pub time: DateTime<Utc>,
    pub url: String,
}

pub async fn scrape_reddit(until: DateTime<Utc>) -> Vec<RedditEntry> {
    let mut results = Vec::new();

    let subreddit = Subreddit::new("rust");

    let mut opts: FeedOption = FeedOption::new();

    'outer: loop {
        let result = subreddit.latest(20, Some(opts.clone())).await.unwrap();

        opts.after = result.data.after.clone();

        for entry in result.data.children {
            let time = DateTime::from_timestamp(entry.data.created_utc as i64, 0).unwrap();

            if time < until {
                break 'outer;
            }

            if entry.data.is_self {
                continue;
            }

            results.push(RedditEntry {
                time,
                url: entry.data.url.unwrap(),
            });
        }
    }

    results
}