use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::ledger_repository::LedgerRepository;
use postings_db::models::ledger::Ledger as DbLedger;
use crate::models::ledger::Ledger as MariaDbLedger;
use postings_db::DbError;

pub struct MariaDbLedgerRepository {
    pool: MySqlPool,
}

impl MariaDbLedgerRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    fn to_domain(mariadb_ledger: MariaDbLedger) -> DbLedger {
        DbLedger {
            id: Uuid::parse_str(&mariadb_ledger.id).unwrap(),
            coa_id: Uuid::parse_str(&mariadb_ledger.coa_id).unwrap(),
        }
    }

    fn from_domain(db_ledger: &DbLedger) -> MariaDbLedger {
        MariaDbLedger {
            id: db_ledger.id.to_string(),
            coa_id: db_ledger.coa_id.to_string(),
        }
    }
}

use uuid::Uuid;

#[async_trait]
impl LedgerRepository for MariaDbLedgerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DbLedger>, DbError> {
        let result: Option<MariaDbLedger> = sqlx::query_as("SELECT * FROM ledger WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        
        Ok(result.map(Self::to_domain))
    }

    async fn save(&self, ledger: &DbLedger) -> Result<(), DbError> {
        let mariadb_ledger = Self::from_domain(ledger);
        sqlx::query("INSERT INTO ledger (id, coa_id) VALUES (?, ?)")
            .bind(mariadb_ledger.id)
            .bind(mariadb_ledger.coa_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
