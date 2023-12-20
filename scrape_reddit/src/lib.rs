use chrono::{DateTime, Utc};
use roux::{
    util::{FeedOption, RouxError},
    Subreddit,
};

///
pub struct RedditEntry {
    pub time: DateTime<Utc>,
    pub url: String,
}

///
pub async fn scrape_reddit(until: DateTime<Utc>) -> Vec<RedditEntry> {
    match scrape_reddit_internal(until).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("scrape_reddit error: {e}");
            Vec::new()
        }
    }
}

///
async fn scrape_reddit_internal(until: DateTime<Utc>) -> Result<Vec<RedditEntry>, RouxError> {
    let mut results = Vec::new();

    let subreddit = Subreddit::new("rust");

    let mut opts: FeedOption = FeedOption::new();

    'outer: loop {
        let result = subreddit.latest(20, Some(opts.clone())).await?;

        opts.after = result.data.after.clone();

        for entry in result.data.children {
            if let Some(time) = DateTime::from_timestamp(entry.data.created_utc as i64, 0) {
                if time > until {
                    if entry.data.is_self {
                        continue;
                    }

                    if let Some(url) = entry.data.url {
                        results.push(RedditEntry { time, url });
                        continue;
                    }
                }
            }

            break 'outer;
        }
    }

    Ok(results)
}
