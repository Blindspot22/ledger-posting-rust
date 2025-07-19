use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::ledger_account_repository::LedgerAccountRepository;
use postings_db::models::ledger_account::LedgerAccount;
use postings_db::DbError;

pub struct PostgresLedgerAccountRepository {
    pool: PgPool,
}

impl PostgresLedgerAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

use uuid::Uuid;

#[async_trait]
impl LedgerAccountRepository for PostgresLedgerAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LedgerAccount>, DbError> {
        sqlx::query_as("SELECT * FROM ledger_account WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger_account: &LedgerAccount) -> Result<(), DbError> {
        sqlx::query("INSERT INTO ledger_account (id, ledger_id, parent_id, coa_id, balance_side, category) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(ledger_account.id)
            .bind(ledger_account.ledger_id)
            .bind(ledger_account.parent_id)
            .bind(ledger_account.coa_id)
            .bind(&ledger_account.balance_side)
            .bind(&ledger_account.category)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
