use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::ledger_repository::LedgerRepository;
use postings_db::models::ledger::Ledger;
use postings_db::DbError;

pub struct MariaDbLedgerRepository {
    pool: MySqlPool,
}

impl MariaDbLedgerRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

use uuid::Uuid;

#[async_trait]
impl LedgerRepository for MariaDbLedgerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ledger>, DbError> {
        sqlx::query_as("SELECT * FROM ledger WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Ledger>, DbError> {
        sqlx::query_as("SELECT * FROM ledger WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger: Ledger) -> Result<Ledger, DbError> {
        sqlx::query("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&ledger.id)
            .bind(&ledger.name)
            .bind(&ledger.coa_id)
            .bind(ledger.created)
            .bind(&ledger.user_details)
            .bind(&ledger.short_desc)
            .bind(&ledger.long_desc)
            .execute(&self.pool)
            .await?;
        Ok(ledger)
    }
}
