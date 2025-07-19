use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;
use crate::domain::ledger_account::LedgerAccount;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostingTrace {
    pub id: Uuid,
    pub tgt_pst_id: Uuid,
    pub src_pst_time: DateTime<Utc>,
    pub src_pst_id: Uuid,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub src_opr_id: [u8; 34],
    pub account: LedgerAccount,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub src_pst_hash: Option<[u8; 34]>,
}
