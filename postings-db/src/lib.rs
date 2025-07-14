pub mod repositories;
pub mod models;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Connection error")]
    Connection,
    #[error("Query error")]
    Query,
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
