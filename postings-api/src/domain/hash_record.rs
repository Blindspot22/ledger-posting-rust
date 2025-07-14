use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HashRecord {
    pub antecedent_id: Option<Uuid>,
    pub antecedent_hash: Option<String>,
    pub hash: Option<String>,
    pub hash_alg: Option<String>,
}
