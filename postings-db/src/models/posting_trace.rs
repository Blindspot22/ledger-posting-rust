use sqlx::FromRow;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingTrace {
    pub id: String,
    pub tgt_pst_id: String,
    pub src_pst_time: chrono::DateTime<chrono::Utc>,
    pub src_pst_id: String,
    pub src_opr_id: String,
    pub account_id: String,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub src_pst_hash: String,
}
