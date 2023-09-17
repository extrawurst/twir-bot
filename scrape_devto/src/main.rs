use crate::model::Article;

mod model;

#[tokio::main]
async fn main() {
    let result: Vec<Article> = reqwest::Client::new()
        .get("https://dev.to/api/articles?tag=rust")
        .header("User-Agent", "rust")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for post in result {
        println!(
            "[{}] ({:4}) {}",
            post.created_at, post.public_reactions_count, post.url
        );
    }
}
