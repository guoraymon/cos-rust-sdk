use dotenv::dotenv;
use std::env;

use cos_rust_sdk::signature::Signature;

pub struct CosClient {
    secret_id: String,
    secret_key: String,
}

impl CosClient {
    pub fn new(secret_id: &str, secret_key: &str) -> CosClient {
        CosClient {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn test(&self) -> String{
        let signature =  Signature::new(&self.secret_id, &self.secret_key);
        signature.create_authorization(7200)
    }
}

fn main() {
    dotenv().ok();

    let cos_client = CosClient::new(&env::var("SECRET_ID").unwrap(), &env::var("SECRET_KEY").unwrap());
    println!("{}", cos_client.test())
}
