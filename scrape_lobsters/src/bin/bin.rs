use chrono::{Duration, Utc};

#[tokio::main]
async fn main() {
    let result = scrape_lobsters::scrape_lobsters(Utc::now() - Duration::days(7)).await;

    for post in result {
        println!("[{}] {}", post.time, post.url);
    }
}
