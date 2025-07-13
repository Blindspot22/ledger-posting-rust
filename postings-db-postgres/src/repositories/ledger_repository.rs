use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::ledger_repository::LedgerRepository;
use postings_db::models::ledger::Ledger;
use postings_db::DbError;

pub struct PostgresLedgerRepository {
    pool: PgPool,
}

impl PostgresLedgerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerRepository for PostgresLedgerRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Ledger>, DbError> {
        sqlx::query_as("SELECT * FROM ledger WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Ledger>, DbError> {
        sqlx::query_as("SELECT * FROM ledger WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger: Ledger) -> Result<Ledger, DbError> {
        sqlx::query_as("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *")
            .bind(ledger.id)
            .bind(ledger.name)
            .bind(ledger.coa_id)
            .bind(ledger.created)
            .bind(ledger.user_details)
            .bind(ledger.short_desc)
            .bind(ledger.long_desc)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
