use serde::{Deserialize, Serialize};
use crate::domain::named::Named;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChartOfAccount {
    #[serde(flatten)]
    pub named: Named,
}
