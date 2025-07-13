use sqlx::Type;

#[derive(Debug, Clone, Type, PartialEq, Eq, Default)]
#[sqlx(type_name = "posting_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostingType {
    #[default]
    BusiTx,
    AdjTx,
    BalStmt,
    PnlStmt,
    BsStmt,
    LdgClsng,
}
