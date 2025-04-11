use bsky::bsky_post;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::args().nth(1).unwrap();

    let bsky_usr = env::var("BSKY_USR")?;
    let bsky_key = env::var("BSKY_KEY")?;

    println!("url provided: {}", url);

    let res = bsky_post(&url, &bsky_usr, &bsky_key).await?;

    println!("output: {:?}", res);

    Ok(())
}
