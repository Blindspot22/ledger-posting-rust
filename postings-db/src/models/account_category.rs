use sqlx::Type;

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "account_category", rename_all = "UPPERCASE")]
pub enum AccountCategory {
    RE,
    EX,
    AS,
    LI,
    EQ,
    NOOP,
    NORE,
    NOEX,
}
