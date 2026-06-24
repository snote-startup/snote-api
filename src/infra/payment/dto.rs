use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PayOSResponse<T> {
    pub code: String,
    pub desc: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequest {
    pub order_code: i64,
    pub amount: i64,
    pub description: String,
    pub cancel_url: String,
    pub return_url: String,
    pub items: Vec<PaymentItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentItem {
    pub name: String,
    pub quantity: i32,
    pub price: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Cancelled,
    Expired,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentResponse {
    pub bin: String,
    pub account_number: String,
    pub account_name: String,
    pub amount: i64,
    pub description: String,
    pub order_code: i64,
    pub currency: String,
    pub payment_link_id: String,
    pub status: PaymentStatus,
    pub checkout_url: String,
    pub qr_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: String,
    pub order_code: i64,
    pub amount: i64,
    pub amount_paid: i64,
    pub amount_remaining: i64,
    pub status: PaymentStatus,
    pub created_at: DateTime<Utc>,
}
