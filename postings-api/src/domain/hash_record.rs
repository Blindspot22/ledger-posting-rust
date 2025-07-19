use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HashRecord {
    pub antecedent_id: Option<Uuid>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub antecedent_hash: Option<[u8; 34]>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub hash: Option<[u8; 34]>,
}
