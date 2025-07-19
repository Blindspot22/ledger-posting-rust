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

use uuid::Uuid;

#[async_trait]
impl LedgerRepository for PostgresLedgerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ledger>, DbError> {
        sqlx::query_as("SELECT * FROM ledger WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger: &Ledger) -> Result<(), DbError> {
        sqlx::query("INSERT INTO ledger (id, coa_id) VALUES ($1, $2)")
            .bind(ledger.id)
            .bind(ledger.coa_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
