use async_trait::async_trait;
use crate::models::ledger::Ledger;
use crate::DbError;

#[async_trait]
pub trait LedgerRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Ledger>, DbError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Ledger>, DbError>;
    async fn save(&self, ledger: Ledger) -> Result<Ledger, DbError>;
}
