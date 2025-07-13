use sqlx::Type;

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "stmt_status", rename_all = "UPPERCASE")]
pub enum StmtStatus {
    Simulated,
    Closed,
}
