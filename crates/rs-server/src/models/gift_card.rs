use async_trait::async_trait;
use rust_decimal::Decimal;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone, sqlx::Type)]
#[sqlx(type_name = "gift_card_status", rename_all = "PascalCase")]
pub enum GiftCardStatus {
    Available,
    Sold,
    Redeemed,
}

#[derive(Debug)]
pub struct GiftCard {
    pub id: Uuid,
    pub amount: Decimal,
    pub status: GiftCardStatus,
    pub created_at: OffsetDateTime,
    pub version: i32,
}

impl GiftCard {
    pub fn new(amount: Decimal) -> Result<Self, GiftCardError> {
        if amount <= Decimal::ZERO {
            return Err(GiftCardError::InvalidAmount(amount));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            amount,
            status: GiftCardStatus::Available,
            created_at: OffsetDateTime::now_utc(),
            version: 0,
        })
    }

    pub fn mark_as_sold(&mut self) -> Result<(), GiftCardError> {
        match self.status {
            GiftCardStatus::Available => {
                self.status = GiftCardStatus::Sold;
                Ok(())
            }
            _ => Err(GiftCardError::NotAvailable {
                id: self.id,
                status: self.status,
            }),
        }
    }

    pub fn mark_as_redeemed(&mut self) -> Result<(), GiftCardError> {
        match self.status {
            GiftCardStatus::Sold => {
                self.status = GiftCardStatus::Redeemed;
                Ok(())
            }
            _ => Err(GiftCardError::NotRedeemable {
                id: self.id,
                status: self.status,
            }),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GiftCardError {
    #[error("gift card {id} is not reedemable, status: {status:?}")]
    NotRedeemable { id: Uuid, status: GiftCardStatus },

    #[error("gift card {id} is not available, status: {status:?}")]
    NotAvailable { id: Uuid, status: GiftCardStatus },

    #[error("invalid gift card amount, got {0})")]
    InvalidAmount(Decimal),
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("{0}")]
    DatabaseError(String),

    #[error("failed updating gift card {id}: version mismatch")]
    OptimisticLockError { id: Uuid },
}

#[async_trait]
pub trait GiftCardRepository {
    async fn save(&self, card: &GiftCard) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<GiftCard>, RepositoryError>;
    async fn update(&self, card: &mut GiftCard) -> Result<(), RepositoryError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn test_gift_card_creation_with_valid_amount() {
        let amount = dec!(100);
        let card = GiftCard::new(amount).unwrap();
        assert_eq!(card.status, GiftCardStatus::Available);
        assert_eq!(card.amount, amount);
    }

    #[test]
    fn test_gift_card_creation_with_zero_amount() {
        let amount = dec!(0);
        let card = GiftCard::new(amount);
        assert!(matches!(card, Err(GiftCardError::InvalidAmount(_))));
    }

    #[test]
    fn test_cannot_sell_already_sold_gift_card() {
        let amount = dec!(100);
        let mut card = GiftCard::new(amount).unwrap();
        card.mark_as_sold().unwrap();
        assert!(matches!(
            card.mark_as_sold(),
            Err(GiftCardError::NotAvailable { .. })
        ));
    }

    #[test]
    fn test_cannot_redeem_unsold_gift_card() {
        let amount = dec!(100);
        let mut card = GiftCard::new(amount).unwrap();
        let result = card.mark_as_redeemed();
        assert!(matches!(result, Err(GiftCardError::NotRedeemable { .. })));
    }

    #[test]
    fn test_happy_path_lifecycle() {
        let amount = dec!(100);
        let mut card = GiftCard::new(amount).unwrap();
        assert_eq!(card.status, GiftCardStatus::Available);
        card.mark_as_sold().unwrap();
        assert_eq!(card.status, GiftCardStatus::Sold);
        card.mark_as_redeemed().unwrap();
        assert_eq!(card.status, GiftCardStatus::Redeemed);
    }
}
