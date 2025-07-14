use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::posting_line_repository::PostingLineRepository;
use postings_db::models::posting_line::PostingLine;
use postings_db::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PostgresPostingLineRepository {
    pool: PgPool,
}

impl PostgresPostingLineRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostingLineRepository for PostgresPostingLineRepository {
    async fn save(&self, posting_line: PostingLine) -> Result<PostingLine, DbError> {
        sqlx::query_as("INSERT INTO posting_line (id, account_id, debit_amount, credit_amount, details, src_account, base_line, sub_opr_src_id, record_time, opr_id, opr_src, pst_time, pst_type, pst_status, hash, discarded_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *")
            .bind(posting_line.id)
            .bind(posting_line.account_id)
            .bind(posting_line.debit_amount)
            .bind(posting_line.credit_amount)
            .bind(posting_line.details)
            .bind(posting_line.src_account)
            .bind(posting_line.base_line)
            .bind(posting_line.sub_opr_src_id)
            .bind(posting_line.record_time)
            .bind(posting_line.opr_id)
            .bind(posting_line.opr_src)
            .bind(posting_line.pst_time)
            .bind(posting_line.pst_type)
            .bind(posting_line.pst_status)
            .bind(posting_line.hash)
            .bind(posting_line.discarded_time)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_account_and_pst_time_between(&self, account_id: Uuid, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE account_id = $1 AND pst_time > $2 AND pst_time <= $3 AND discarded_time IS NULL ORDER BY pst_time DESC")
            .bind(account_id)
            .bind(from)
            .bind(to)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id_and_account_id(&self, id: Uuid, account_id: Uuid) -> Result<Option<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE id = $1 AND account_id = $2")
            .bind(id)
            .bind(account_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }
    
    async fn find_by_base_line_and_pst_time_less_than_equal(&self, base_line: &str, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE base_line = $1 AND pst_time <= $2 AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(base_line)
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_account_and_pst_time_less_than_equal(&self, account_id: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE account_id = $1 AND pst_time <= $2 AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(account_id)
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
