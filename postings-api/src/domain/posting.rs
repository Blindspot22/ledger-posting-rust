use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::hash_record::HashRecord;
use crate::domain::ledger::Ledger;
use crate::domain::posting_line::PostingLine;
use crate::domain::posting_status::PostingStatus;
use crate::domain::posting_type::PostingType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Posting {
    pub id: Uuid,
    pub record_user: String,
    pub record_time: DateTime<Utc>,
    pub opr_id: String,
    pub opr_time: DateTime<Utc>,
    pub opr_type: String,
    pub opr_details: String,
    pub opr_src: Option<String>,
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
