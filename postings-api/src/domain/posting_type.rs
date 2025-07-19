use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PostingType {
    BusiTx,
    AdjTx,
    BalStmt,
    PnLStmt,
    BsStmt,
    LdgClsng,
    Unknown,
}
