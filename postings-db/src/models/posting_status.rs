use sqlx::Type;

#[derive(Debug, Clone, Type, PartialEq, Eq, Default)]
#[sqlx(type_name = "posting_status", rename_all = "UPPERCASE")]
pub enum PostingStatus {
    Deferred,
    #[default]
    Posted,
    Proposed,
    Simulated,
    Tax,
    Unposted,
    Cancelled,
    Other,
}
