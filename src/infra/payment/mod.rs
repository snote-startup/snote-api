mod dto;

const API_URL: &str = "https://api-merchant.payos.vn";

pub struct PayOSClient {
    client_id: String,
    api_key: String,
    checksum_key: String,
    http: reqwest::Client,
}

impl PayOSClient {
    pub fn new(client_id: String, api_key: String, checksum_key: String) -> Self {
        Self {
            client_id,
            api_key,
            checksum_key,
            http: reqwest::Client::new(),
        }
    }
}
