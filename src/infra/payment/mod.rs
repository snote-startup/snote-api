pub mod dto;

use aws_lc_rs::hmac;

use dto::{CreatePaymentRequest, CreatePaymentResponse, PayOSResponse, Payment};

use crate::infra::payment::dto::PaymentItem;

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

    pub fn payment_signature(
        &self,
        amount: i64,
        cancel_url: &str,
        description: &str,
        order_code: i64,
        return_url: &str,
    ) -> String {
        let raw = format!(
            "amount={amount}&cancelUrl={cancel_url}&description={description}&orderCode={order_code}&returnUrl={return_url}"
        );
        self.hmac(&raw)
    }

    fn hmac(&self, data: &str) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, self.checksum_key.as_bytes());
        let tag = hmac::sign(&key, data.as_bytes());
        hex::encode(tag.as_ref())
    }

    pub async fn create_payment_link(
        &self,
        amount: i64,
        description: &str,
        cancel_url: &str,
        return_url: &str,
        items: Vec<PaymentItem>,
        expired_at: Option<i64>,
    ) -> color_eyre::Result<CreatePaymentResponse> {
        let order_code = generate_order_code();

        let signature =
            self.payment_signature(amount, cancel_url, description, order_code, return_url);

        let req = CreatePaymentRequest {
            order_code,
            amount,
            description: description.into(),
            cancel_url: cancel_url.into(),
            return_url: return_url.into(),
            items,
            expired_at,
            signature,
        };

        let resp: PayOSResponse<CreatePaymentResponse> = self
            .http
            .post(format!("{}/v2/payment-requests", API_URL))
            .header("x-client-id", &self.client_id)
            .header("x-api-key", &self.api_key)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        self.unwrap(resp)
    }

    pub async fn get_payment_info(&self, order_code: i64) -> color_eyre::Result<Payment> {
        let resp: PayOSResponse<Payment> = self
            .http
            .get(format!("{}/v2/payment-requests/{}", API_URL, order_code))
            .header("x-client-id", &self.client_id)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .json()
            .await?;

        self.unwrap(resp)
    }

    fn unwrap<T>(&self, resp: PayOSResponse<T>) -> color_eyre::Result<T> {
        if resp.code != "00" {
            return Err(color_eyre::eyre::anyhow!(
                "PayOS [{}]: {}",
                resp.code,
                resp.desc
            ));
        }
        resp.data
            .ok_or_else(|| color_eyre::eyre::anyhow!("PayOS returned empty data"))
    }
}

fn generate_order_code() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let nonce = rand::random_range(0..999i64);
    secs * 1000 + nonce
}
