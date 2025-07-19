use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use type_rules::prelude::*;
use uuid::Uuid;
use crate::domain::hash_record::HashRecord;
use crate::domain::ledger::Ledger;
use crate::domain::posting_line::PostingLine;
use crate::domain::posting_status::PostingStatus;
use crate::domain::posting_type::PostingType;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validator)]
pub struct Posting {
    pub id: Uuid,
    /// 32-byte hash of name of User that recorded the posting
    #[serde_as(as = "serde_with::hex::Hex")]
    pub record_user: [u8; 34],
    pub record_time: DateTime<Utc>,
    /// 32-byte hash of operation id
    #[serde_as(as = "serde_with::hex::Hex")]
    pub opr_id: [u8; 34],
    pub opr_time: DateTime<Utc>,
    /// 32-byte hash of Operation Type
    #[serde_as(as = "serde_with::hex::Hex")]
    pub opr_type: [u8; 34],
    /// 32-byte hash of Operation Details
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub opr_details: Option<[u8; 34]>,
    /// 32-byte hash Operation Source
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub opr_src: Option<[u8; 34]>,
    pub pst_time: DateTime<Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub ledger: Ledger,
    pub val_time: Option<DateTime<Utc>>,
    pub lines: Vec<PostingLine>,
    pub discarded_id: Option<Uuid>,
    pub discarded_time: Option<DateTime<Utc>>,
    pub discarding_id: Option<Uuid>,
    #[serde(flatten)]
    pub hash_record: HashRecord,
}

