use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::posting::Posting;
use crate::domain::posting_trace::PostingTrace;
use crate::domain::stmt_status::StmtStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancialStmt {
    pub id: Uuid,
    pub posting: Option<Posting>,
    pub pst_time: DateTime<Utc>,
    pub stmt_status: StmtStatus,
    pub latest_pst: Option<PostingTrace>,
    pub stmt_seq_nbr: i32,
}
