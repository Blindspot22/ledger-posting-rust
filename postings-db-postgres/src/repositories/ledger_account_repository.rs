use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::ledger_account_repository::LedgerAccountRepository;
use postings_db::models::ledger_account::LedgerAccount;
use postings_db::DbError;
use std::collections::HashSet;

pub struct PostgresLedgerAccountRepository {
    pool: PgPool,
}

impl PostgresLedgerAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerAccountRepository for PostgresLedgerAccountRepository {
    async fn find_by_id(&self, id: &str) -> Result<LedgerAccount, DbError> {
        sqlx::query_as("SELECT * FROM ledger_account WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_ledger_and_name(&self, ledger_id: &str, name: &str) -> Result<Option<LedgerAccount>, DbError> {
        sqlx::query_as("SELECT * FROM ledger_account WHERE ledger_id = $1 AND name = $2")
            .bind(ledger_id)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, ledger_account: LedgerAccount) -> Result<LedgerAccount, DbError> {
        sqlx::query_as("INSERT INTO ledger_account (id, name, ledger_id, parent_id, coa_id, balance_side, category, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *")
            .bind(ledger_account.id)
            .bind(ledger_account.name)
            .bind(ledger_account.ledger_id)
            .bind(ledger_account.parent_id)
            .bind(ledger_account.coa_id)
            .bind(ledger_account.balance_side)
            .bind(ledger_account.category)
            .bind(ledger_account.created)
            .bind(ledger_account.user_details)
            .bind(ledger_account.short_desc)
            .bind(ledger_account.long_desc)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_ibans(&self, ibans: HashSet<String>, ledger_id: &str) -> Result<Vec<LedgerAccount>, DbError> {
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM ledger_account WHERE ledger_id = ");
        query_builder.push_bind(ledger_id);
        query_builder.push(" AND name IN (");
        let mut separated = query_builder.separated(", ");
        for iban in ibans {
            separated.push_bind(iban);
        }
        separated.push_unseparated(")");
        let query = query_builder.build_query_as();
        query.fetch_all(&self.pool).await.map_err(DbError::from)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{QueryBuilder, Postgres};
    use std::collections::HashSet;

    #[test]
    fn test_find_by_ibans_query_builder_single_iban() {
        let ledger_id = "test_ledger";
        let mut ibans = HashSet::new();
        ibans.insert("IBAN1".to_string());

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM ledger_account WHERE ledger_id = ");
        query_builder.push_bind(ledger_id);
        query_builder.push(" AND name IN (");
        let mut separated = query_builder.separated(", ");
        for iban in ibans {
            separated.push_bind(iban);
        }
        separated.push_unseparated(")");
        
        let sql = query_builder.into_sql();
        assert_eq!(sql, "SELECT * FROM ledger_account WHERE ledger_id = $1 AND name IN ($2)");
    }

    #[test]
    fn test_find_by_ibans_query_builder_multiple_ibans() {
        let ledger_id = "test_ledger";
        let mut ibans = HashSet::new();
        ibans.insert("IBAN1".to_string());
        ibans.insert("IBAN2".to_string());

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM ledger_account WHERE ledger_id = ");
        query_builder.push_bind(ledger_id);
        query_builder.push(" AND name IN (");
        let mut separated = query_builder.separated(", ");
        for iban in ibans {
            separated.push_bind(iban);
        }
        separated.push_unseparated(")");
        
        let sql = query_builder.into_sql();
        // The order of elements in a HashSet is not guaranteed, so we check for both possibilities.
        let option1 = "SELECT * FROM ledger_account WHERE ledger_id = $1 AND name IN ($2, $3)";
        let option2 = "SELECT * FROM ledger_account WHERE ledger_id = $1 AND name IN ($3, $2)";
        assert!(sql == option1 || sql == option2);
    }

    #[test]
    fn test_find_by_ibans_query_builder_no_ibans() {
        let ledger_id = "test_ledger";
        let ibans = HashSet::<String>::new();

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM ledger_account WHERE ledger_id = ");
        query_builder.push_bind(ledger_id);
        query_builder.push(" AND name IN (");
        let mut separated = query_builder.separated(", ");
        for iban in ibans {
            separated.push_bind(iban);
        }
        separated.push_unseparated(")");
        
        let sql = query_builder.into_sql();
        assert_eq!(sql, "SELECT * FROM ledger_account WHERE ledger_id = $1 AND name IN ()");
    }
}
