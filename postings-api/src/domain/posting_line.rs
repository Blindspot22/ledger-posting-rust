use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use type_rules::prelude::*;
use uuid::Uuid;
use crate::domain::ledger_account::LedgerAccount;
use crate::domain::posting_status::PostingStatus;
use crate::domain::posting_type::PostingType;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validator)]
pub struct PostingLine {
    pub id: Uuid,
    pub account: LedgerAccount,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub details: Option<[u8; 34]>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub src_account: Option<[u8; 34]>,
    pub base_line: Option<Uuid>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub sub_opr_src_id: Option<[u8; 34]>,
    pub record_time: DateTime<Utc>,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub opr_id: [u8; 34],
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub opr_src: Option<[u8; 34]>,
    pub pst_time: DateTime<Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub hash: Option<[u8; 34]>,
    #[rule(Opt(MaxLength(1024)))]
    pub additional_information: Option<String>,
    pub discarded_time: Option<DateTime<Utc>>,
}
