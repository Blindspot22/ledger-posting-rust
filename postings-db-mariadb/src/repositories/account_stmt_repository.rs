use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::account_stmt_repository::AccountStmtRepository;
use postings_db::models::account_stmt::AccountStmt;
use postings_db::models::stmt_status::StmtStatus;
use postings_db::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct MariaDbAccountStmtRepository {
    pool: MySqlPool,
}

impl MariaDbAccountStmtRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountStmtRepository for MariaDbAccountStmtRepository {
    async fn find_first_by_account_and_status_and_pst_time_less_than_ordered(&self, account_id: Uuid, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError> {
        sqlx::query_as("SELECT * FROM account_stmt WHERE account_id = ? AND stmt_status = ? AND pst_time < ? ORDER BY pst_time DESC, stmt_seq_nbr DESC LIMIT 1")
            .bind(account_id.to_string())
            .bind(status)
            .bind(ref_time)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_first_by_account_and_status_and_pst_time_greater_than_equal(&self, account_id: Uuid, status: StmtStatus, ref_time: DateTime<Utc>) -> Result<Option<AccountStmt>, DbError> {
        sqlx::query_as("SELECT * FROM account_stmt WHERE account_id = ? AND stmt_status = ? AND pst_time >= ? LIMIT 1")
            .bind(account_id.to_string())
            .bind(status)
            .bind(ref_time)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, stmt: AccountStmt) -> Result<AccountStmt, DbError> {
        sqlx::query("INSERT INTO account_stmt (id, account_id, youngest_pst_id, total_debit, total_credit, posting_id, pst_time, stmt_status, latest_pst_id, stmt_seq_nbr) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(stmt.id.to_string())
            .bind(stmt.account_id.to_string())
            .bind(stmt.youngest_pst_id.map(|u| u.to_string()))
            .bind(&stmt.total_debit)
            .bind(&stmt.total_credit)
            .bind(stmt.posting_id.map(|u| u.to_string()))
            .bind(stmt.pst_time)
            .bind(&stmt.stmt_status)
            .bind(stmt.latest_pst_id.map(|u| u.to_string()))
            .bind(stmt.stmt_seq_nbr)
            .execute(&self.pool)
            .await?;
        Ok(stmt)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AccountStmt>, DbError> {
        sqlx::query_as("SELECT * FROM account_stmt WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }
}