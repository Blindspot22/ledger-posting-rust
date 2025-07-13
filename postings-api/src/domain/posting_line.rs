use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::ledger_account::LedgerAccount;
use crate::domain::posting_status::PostingStatus;
use crate::domain::posting_type::PostingType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostingLine {
    pub id: Uuid,
    pub account: LedgerAccount,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub details: String,
    pub src_account: Option<String>,
    pub base_line: Option<String>,
    pub sub_opr_src_id: Option<String>,
    pub record_time: DateTime<Utc>,
    pub opr_id: String,
    pub opr_src: Option<String>,
    pub pst_time: DateTime<Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub hash: String,
    pub additional_information: Option<String>,
    pub discarded_time: Option<DateTime<Utc>>,
}
