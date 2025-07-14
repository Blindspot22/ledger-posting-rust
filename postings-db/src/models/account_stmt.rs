use uuid::Uuid;
use sqlx::FromRow;
use bigdecimal::BigDecimal;
use crate::models::stmt_status::StmtStatus;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct AccountStmt {
    pub id: Uuid,
    pub account_id: Uuid,
    pub youngest_pst_id: Option<Uuid>,
    pub total_debit: BigDecimal,
    pub total_credit: BigDecimal,
    pub posting_id: Option<Uuid>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub stmt_status: StmtStatus,
    pub latest_pst_id: Option<Uuid>,
    pub stmt_seq_nbr: i32,
}
