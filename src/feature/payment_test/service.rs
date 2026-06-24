use dashmap::DashMap;
use http::StatusCode;
use uuid::Uuid;

use crate::{
    error::{ErrorContext, Result},
    infra::payment::{
        PayOSClient,
        dto::{PaymentItem, PaymentStatus},
    },
};

pub struct PaymentTestService {
    cache: DashMap<i64, Uuid>,
    base_url: String,
}

impl PaymentTestService {
    pub fn new(base_url: String) -> Self {
        Self {
            cache: Default::default(),
            base_url,
        }
    }

    #[tracing::instrument(err(Debug), skip(self, payos))]
    pub async fn create_link(
        &self,

        payos: &PayOSClient,

        account_id: Uuid,
        amount: u32,
    ) -> Result<String> {
        let resp = payos
            .create_payment_link(
                amount as i64,
                "Test Product",
                &format!("{}/payment-test/cancel", self.base_url),
                &format!("{}/payment-test/return", self.base_url),
                vec![PaymentItem {
                    name: "Test Product".to_string(),
                    quantity: 1,
                    price: amount as i64,
                }],
                None,
            )
            .await?;
        self.cache.insert(resp.order_code, account_id);

        Ok(resp.checkout_url)
    }

    pub async fn handle_return(&self, payos: &PayOSClient, order_code: i64) -> Result<()> {
        let Some(account_id) = self.cache.get(&order_code) else {
            return Err(ErrorContext {
                status: StatusCode::FORBIDDEN,
                message: "Invalid payment".to_string(),
                ..Default::default()
            }
            .into());
        };
        let account_id = account_id.value();

        let payment = payos.get_payment_info(order_code).await?;
        if payment.status != PaymentStatus::Paid {
            return Err(ErrorContext {
                status: StatusCode::FORBIDDEN,
                message: "Item is not paid".to_string(),
                ..Default::default()
            }
            .into());
        }

        tracing::info!("Order {} of account {} is paid", order_code, account_id);

        Ok(())
    }
}
