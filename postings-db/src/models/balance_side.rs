use sqlx::Type;

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "balance_side")]
pub enum BalanceSide {
    Dr,
    Cr,
    DrCr,
}
