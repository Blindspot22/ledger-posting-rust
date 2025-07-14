use async_trait::async_trait;
use crate::models::account_stmt::AccountStmt;
use crate::models::stmt_status::StmtStatus;
use crate::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait AccountStmtRepository {
    async fn find_first_by_account_and_status_and_pst_time_less_than_ordered(&self, account_id: Uuid, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError>;
    async fn find_first_by_account_and_status_and_pst_time_greater_than_equal(&self, account_id: Uuid, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError>;
    async fn save(&self, stmt: AccountStmt) -> Result<AccountStmt, DbError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AccountStmt>, DbError>;
}
