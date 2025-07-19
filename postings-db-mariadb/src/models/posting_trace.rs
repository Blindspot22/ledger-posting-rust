use bigdecimal::BigDecimal;
use sqlx::FromRow;
use uuid::Uuid;
use postings_db::models::posting_trace::PostingTrace;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingTraceDb {
    pub id: Uuid,
    pub tgt_pst_id: Uuid,
    pub src_pst_time: chrono::DateTime<chrono::Utc>,
    pub src_pst_id: Uuid,
    pub src_opr_id: Vec<u8>,
    pub account_id: Uuid,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub src_pst_hash: Option<Vec<u8>>,
}

impl From<PostingTraceDb> for PostingTrace {
    fn from(p: PostingTraceDb) -> Self {
        Self {
            id: p.id,
            tgt_pst_id: p.tgt_pst_id,
            src_pst_time: p.src_pst_time,
            src_pst_id: p.src_pst_id,
            src_opr_id: p.src_opr_id.try_into().unwrap_or([0u8; 34]),
            account_id: p.account_id,
            debit_amount: p.debit_amount,
            credit_amount: p.credit_amount,
            src_pst_hash: p.src_pst_hash.map(|v| v.try_into().unwrap_or([0u8; 34])),
        }
    }
}

impl From<PostingTrace> for PostingTraceDb {
    fn from(p: PostingTrace) -> Self {
        Self {
            id: p.id,
            tgt_pst_id: p.tgt_pst_id,
            src_pst_time: p.src_pst_time,
            src_pst_id: p.src_pst_id,
            src_opr_id: p.src_opr_id.to_vec(),
            account_id: p.account_id,
            debit_amount: p.debit_amount,
            credit_amount: p.credit_amount,
            src_pst_hash: p.src_pst_hash.map(|v| v.to_vec()),
        }
    }
}