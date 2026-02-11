use crate::models::{GiftCard, GiftCardRepository, GiftCardStatus, RepositoryError};
use async_trait::async_trait;
use uuid::Uuid;

pub struct PostgresRepository {
    pool: sqlx::PgPool,
}

impl PostgresRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GiftCardRepository for PostgresRepository {
    async fn save(&self, card: &GiftCard) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"INSERT INTO rs_market.gift_cards (id, amount, status, created_at, version) 
            VALUES ($1, $2, $3, $4, $5)"#,
            card.id,
            card.amount,
            card.status as GiftCardStatus,
            card.created_at,
            card.version
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(format!("failed to insert gift card {}: {}", card.id, e))
        })?;
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<GiftCard>, RepositoryError> {
        sqlx::query_as!(
            GiftCard,
            r#"SELECT id, amount, status as "status: GiftCardStatus", created_at, version 
            FROM rs_market.gift_cards 
            WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(format!("gift card {} not found: {}", id, e)))
    }

    async fn update(&self, card: &mut GiftCard) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"
                UPDATE rs_market.gift_cards
                SET status = $2,
                version = version + 1
                WHERE id = $1
                AND version = $3
                RETURNING version
            "#,
            card.id,
            card.status as GiftCardStatus,
            card.version
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(format!("couldn't update gift card {}: {}", card.id, e))
        })?
        .ok_or(RepositoryError::OptimisticLockError { id: card.id })?;

        card.version = result.version;

        Ok(())
    }
}
