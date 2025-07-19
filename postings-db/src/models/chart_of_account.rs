use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct ChartOfAccount {
    pub id: Uuid,
}
