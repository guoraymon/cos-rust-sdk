pub struct CosClient {
    pub(crate) secret_id: String,
    pub(crate) secret_key: String,
}

impl CosClient {
    pub fn new(secret_id: &str, secret_key: &str) -> CosClient {
        CosClient {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
        }
    }
}
