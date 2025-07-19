use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::ledger_account_repository::LedgerAccountRepository;
use postings_db::models::ledger_account::LedgerAccount;
use postings_db::DbError;
use std::collections::HashSet;

pub struct MariaDbLedgerAccountRepository {
    pool: MySqlPool,
}

impl MariaDbLedgerAccountRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

use uuid::Uuid;

#[async_trait]
impl LedgerAccountRepository for MariaDbLedgerAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LedgerAccount>, DbError> {
        sqlx::query_as("SELECT * FROM ledger_account WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger_account: &LedgerAccount) -> Result<(), DbError> {
        sqlx::query("INSERT INTO ledger_account (id, ledger_id, parent_id, coa_id, balance_side, category) VALUES (?, ?, ?, ?, ?, ?)")
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