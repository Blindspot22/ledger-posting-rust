use async_trait::async_trait;
use crate::models::ledger_account::LedgerAccount;
use crate::DbError;
use uuid::Uuid;

#[async_trait]
pub trait LedgerAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LedgerAccount>, DbError>;
    async fn save(&self, ledger_account: &LedgerAccount) -> Result<(), DbError>;
}
