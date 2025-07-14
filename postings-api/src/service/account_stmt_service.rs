use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::account_stmt::AccountStmt;
use crate::domain::ledger_account::LedgerAccount;
use crate::ServiceError;

#[async_trait]
pub trait AccountStmtService {
    async fn read_stmt(&self, ledger_account: LedgerAccount, ref_time: DateTime<Utc>) -> Result<AccountStmt, ServiceError>;
    async fn create_stmt(&self, ledger_account: LedgerAccount, ref_time: DateTime<Utc>) -> Result<AccountStmt, ServiceError>;
    async fn close_stmt(&self, stmt: AccountStmt) -> Result<AccountStmt, ServiceError>;
}
