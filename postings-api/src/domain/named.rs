use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use type_rules::prelude::*;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validator)]
pub struct Named {
    pub id: Uuid,
    pub container: Uuid,
    pub context: Uuid,
    #[rule(MaxLength(255))]
    pub name: String,
    #[rule(RegEx(r"^[a-z]{2}$"))]
    pub language: String,
    pub created: DateTime<Utc>,
    /// 32-byte hash of the lowercase string of user details.
    #[serde_as(as = "serde_with::hex::Hex")]
    pub user_details: [u8; 34],
    #[rule(Opt(MaxLength(1024)))]
    pub short_desc: Option<String>,
    #[rule(Opt(MaxLength(2048)))]
    pub long_desc: Option<String>,
    pub container_type: ContainerType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContainerType {
    ChartOfAccount,
    Ledger,
    LedgerAccount,
}