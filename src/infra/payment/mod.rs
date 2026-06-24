pub mod dto;

use aws_lc_rs::hmac;

use dto::{CreatePaymentRequest, CreatePaymentResponse, PayOsResponse, Payment};

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
        req: &CreatePaymentRequest,
    ) -> color_eyre::Result<CreatePaymentResponse> {
        let signature = self.payment_signature(
            req.amount,
            &req.cancel_url,
            &req.description,
            req.order_code,
            &req.return_url,
        );

        let resp: PayOsResponse<CreatePaymentResponse> = self
            .http
            .post(format!("{}/v2/payment-requests", API_URL))
            .header("x-client-id", &self.client_id)
            .header("x-api-key", &self.api_key)
            .json(req)
            .send()
            .await?
            .json()
            .await?;

        self.unwrap(resp)
    }

    pub async fn get_payment_info(&self, order_code: i64) -> color_eyre::Result<Payment> {
        let resp: PayOsResponse<Payment> = self
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

    fn unwrap<T>(&self, resp: PayOsResponse<T>) -> color_eyre::Result<T> {
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
