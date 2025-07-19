use uuid::Uuid;
use sqlx::FromRow;
use bigdecimal::BigDecimal;
use crate::models::posting_type::PostingType;
use crate::models::posting_status::PostingStatus;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingLine {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub details: Option<[u8; 34]>,
    pub src_account: Option<[u8; 34]>,
    pub base_line: Option<Uuid>,
    pub sub_opr_src_id: Option<[u8; 34]>,
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: [u8; 34],
    pub opr_src: Option<[u8; 34]>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub hash: Option<[u8; 34]>,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for PostingLine {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            account_id: Uuid::nil(),
            debit_amount: BigDecimal::from(0),
            credit_amount: BigDecimal::from(0),
            details: None,
            src_account: None,
            base_line: None,
            sub_opr_src_id: None,
            record_time: chrono::Utc::now(),
            opr_id: [0; 34],
            opr_src: None,
            pst_time: chrono::Utc::now(),
            pst_type: Default::default(),
            pst_status: Default::default(),
            hash: None,
            discarded_time: None,
        }
    }
}
