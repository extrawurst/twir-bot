use chrono::{Duration, Utc};

#[tokio::main]
async fn main() {
    let results = scrape_reddit::scrape_reddit(Utc::now() - Duration::days(7)).await;

    for entry in results {
        println!("[{}] {:?}", entry.time, entry.url)
    }
}
