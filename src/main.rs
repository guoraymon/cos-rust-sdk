use cos_rust_sdk::client::CosClient;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cos_client = CosClient::new(
        &env::var("SECRET_ID").unwrap(),
        &env::var("SECRET_KEY").unwrap(),
    );
    let list_buckets = cos_client.list_bucket().await.unwrap();
    dbg!(list_buckets);
}
