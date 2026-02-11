use crate::models::{GiftCard, GiftCardError, GiftCardRepository, RepositoryError};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error(transparent)]
    Domain(#[from] GiftCardError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error("gift card with id {0} not found")]
    NotFound(Uuid),
}

pub struct GiftCardService<R: GiftCardRepository> {
    repository: R,
}

impl<R: GiftCardRepository> GiftCardService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_gift_card(&self, amount: Decimal) -> Result<GiftCard, ServiceError> {
        let card = GiftCard::new(amount)?;
        self.repository.save(&card).await?;
        Ok(card)
    }

    pub async fn buy_gift_card(&self, id: Uuid) -> Result<GiftCard, ServiceError> {
        let mut card = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound(id))?;

        card.mark_as_sold()?;
        self.repository.update(&mut card).await?;
        Ok(card)
    }

    pub async fn redeem_gift_card(&self, id: Uuid) -> Result<GiftCard, ServiceError> {
        let mut card = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound(id))?;

        card.mark_as_redeemed()?;
        self.repository.update(&mut card).await?;
        Ok(card)
    }
}
