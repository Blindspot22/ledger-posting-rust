use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct OperationDetails {
    pub id: String,
    pub op_details: String,
}
