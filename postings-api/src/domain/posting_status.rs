use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PostingStatus {
    Deferred,
    Posted,
    Proposed,
    Simulated,
    Tax,
    Unposted,
    Cancelled,
    Other,
}
