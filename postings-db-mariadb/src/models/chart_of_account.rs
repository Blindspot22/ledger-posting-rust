use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct ChartOfAccount {
    pub id: String,
}