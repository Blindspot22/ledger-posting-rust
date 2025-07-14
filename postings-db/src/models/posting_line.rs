use uuid::Uuid;
use sqlx::FromRow;
use bigdecimal::BigDecimal;
use crate::models::posting_type::PostingType;
use crate::models::posting_status::PostingStatus;

#[derive(Debug, Clone, FromRow, PartialEq, Default)]
pub struct PostingLine {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub details: Option<String>,
    pub src_account: Option<String>,
    pub base_line: Option<String>,
    pub sub_opr_src_id: Option<String>,
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: String,
    pub opr_src: Option<String>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub hash: String,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
}
