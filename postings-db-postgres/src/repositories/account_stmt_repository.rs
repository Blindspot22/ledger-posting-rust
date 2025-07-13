use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::account_stmt_repository::AccountStmtRepository;
use postings_db::models::account_stmt::AccountStmt;
use postings_db::models::stmt_status::StmtStatus;
use postings_db::DbError;
use chrono::{DateTime, Utc};

pub struct PostgresAccountStmtRepository {
    pool: PgPool,
}

impl PostgresAccountStmtRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountStmtRepository for PostgresAccountStmtRepository {
    async fn find_first_by_account_and_status_and_pst_time_less_than_ordered(&self, account_id: &str, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError> {
        sqlx::query_as("SELECT * FROM account_stmt WHERE account_id = $1 AND stmt_status = $2 AND pst_time < $3 ORDER BY pst_time DESC, stmt_seq_nbr DESC LIMIT 1")
            .bind(account_id)
            .bind(status)
            .bind(ref_time)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_first_by_account_and_status_and_pst_time_greater_than_equal(&self, account_id: &str, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError> {
        sqlx::query_as("SELECT * FROM account_stmt WHERE account_id = $1 AND stmt_status = $2 AND pst_time >= $3 LIMIT 1")
            .bind(account_id)
            .bind(status)
            .bind(ref_time)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, stmt: AccountStmt) -> Result<AccountStmt, DbError> {
        sqlx::query_as("INSERT INTO account_stmt (id, account_id, youngest_pst_id, total_debit, total_credit, posting_id, pst_time, stmt_status, latest_pst_id, stmt_seq_nbr) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
            .bind(stmt.id)
            .bind(stmt.account_id)
            .bind(stmt.youngest_pst_id)
            .bind(stmt.total_debit)
            .bind(stmt.total_credit)
            .bind(stmt.posting_id)
            .bind(stmt.pst_time)
            .bind(stmt.stmt_status)
            .bind(stmt.latest_pst_id)
            .bind(stmt.stmt_seq_nbr)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
