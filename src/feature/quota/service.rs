use dashmap::DashMap;
use http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ErrorContext, Result},
    feature::quota::repository,
    infra::payment::{
        PayOSClient,
        dto::{PaymentItem, PaymentStatus},
    },
};

const PREMIUM_PRICE: i64 = 70000;
const PREMIUM_QUOTA: i32 = 20;

pub struct QuotaService {
    cache: DashMap<i64, Uuid>,
    base_url: String,
}

impl QuotaService {
    pub fn new(base_url: String) -> Self {
        Self {
            cache: Default::default(),
            base_url,
        }
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn get(&self, db: &PgPool, account_id: Uuid) -> Result<i32> {
        let quota = repository::get_quota(db, account_id)
            .await
            .map(|x| x.max(0))?;

        Ok(quota)
    }

    #[tracing::instrument(err(Debug), skip(self, payos))]
    pub async fn buy(&self, payos: &PayOSClient, account_id: Uuid) -> Result<String> {
        let resp = payos
            .create_payment_link(
                PREMIUM_PRICE,
                "Premium",
                &format!("{}/payment-test/cancel", self.base_url),
                &format!("{}/payment-test/return", self.base_url),
                vec![PaymentItem {
                    name: "Premium".to_string(),
                    quantity: 1,
                    price: PREMIUM_PRICE,
                }],
                None,
            )
            .await?;
        self.cache.insert(resp.order_code, account_id);

        Ok(resp.checkout_url)
    }

    #[tracing::instrument(err(Debug), skip(self, db, payos))]
    pub async fn handle_payment_return(
        &self,

        db: &PgPool,
        payos: &PayOSClient,

        order_code: i64,
    ) -> Result<()> {
        let Some(account_id) = self.cache.get(&order_code) else {
            return Err(ErrorContext {
                status: StatusCode::FORBIDDEN,
                message: "Invalid payment".to_string(),
                ..Default::default()
            }
            .into());
        };
        let account_id = *account_id.value();

        let payment = payos.get_payment_info(order_code).await?;
        if payment.status != PaymentStatus::Paid {
            return Err(ErrorContext {
                status: StatusCode::FORBIDDEN,
                message: "Item is not paid".to_string(),
                ..Default::default()
            }
            .into());
        }

        let mut transaction = db.begin().await?;
        repository::create_quota(&mut *transaction, account_id).await?;
        repository::update_quota(&mut *transaction, account_id, PREMIUM_QUOTA).await?;
        transaction.commit().await?;

        Ok(())
    }

    #[tracing::instrument(err(Debug), skip(self, db))]
    pub async fn decrease(&self, db: &PgPool, account_id: Uuid) -> Result<()> {
        let mut transaction = db.begin().await?;
        repository::create_quota(&mut *transaction, account_id).await?;
        repository::update_quota(&mut *transaction, account_id, -1).await?;
        transaction.commit().await?;

        Ok(())
    }
}
