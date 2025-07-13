use async_trait::async_trait;
use crate::models::ledger_account::LedgerAccount;
use crate::DbError;
use std::collections::HashSet;

#[async_trait]
pub trait LedgerAccountRepository {
    async fn find_by_id(&self, id: &str) -> Result<LedgerAccount, DbError>;
    async fn find_by_ledger_and_name(&self, ledger_id: &str, name: &str) -> Result<Option<LedgerAccount>, DbError>;
    async fn save(&self, ledger_account: LedgerAccount) -> Result<LedgerAccount, DbError>;
    async fn find_by_ibans(&self, ibans: HashSet<String>, ledger_id: &str) -> Result<Vec<LedgerAccount>, DbError>;
}
