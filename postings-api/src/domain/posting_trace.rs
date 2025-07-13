use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::ledger_account::LedgerAccount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostingTrace {
    pub id: Uuid,
    pub tgt_pst_id: Uuid,
    pub src_pst_time: DateTime<Utc>,
    pub src_pst_id: Uuid,
    pub src_opr_id: String,
    pub account: LedgerAccount,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub src_pst_hash: String,
}
