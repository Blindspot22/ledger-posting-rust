use bigdecimal::BigDecimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingTrace {
    pub id: Uuid,
    pub tgt_pst_id: Uuid,
    pub src_pst_time: chrono::DateTime<chrono::Utc>,
    pub src_pst_id: Uuid,
    pub src_opr_id: String,
    pub account_id: Uuid,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub src_pst_hash: String,
}
