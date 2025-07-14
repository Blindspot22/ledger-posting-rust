use async_trait::async_trait;
use crate::models::ledger::Ledger;
use crate::DbError;
use uuid::Uuid;

#[async_trait]
pub trait LedgerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ledger>, DbError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Ledger>, DbError>;
    async fn save(&self, ledger: Ledger) -> Result<Ledger, DbError>;
}
