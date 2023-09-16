use crate::model::Post;

mod model;

#[tokio::main]
async fn main() {
    let newest: Vec<Post> = reqwest::Client::new()
        .get("https://lobste.rs/t/rust.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for post in newest {
        println!("[{}] {}", post.created_at, post.url);
    }
}
