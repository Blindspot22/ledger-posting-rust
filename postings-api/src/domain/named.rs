use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Named {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub user_details: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}
