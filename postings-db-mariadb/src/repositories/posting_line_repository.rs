use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::posting_line_repository::PostingLineRepository;
use postings_db::models::posting_line::PostingLine;
use postings_db::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct MariaDbPostingLineRepository {
    pool: MySqlPool,
}

impl MariaDbPostingLineRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostingLineRepository for MariaDbPostingLineRepository {
    async fn save(&self, _posting_line: PostingLine) -> Result<PostingLine, DbError> {
        todo!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_account_and_pst_time_between(&self, account_id: Uuid, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE account_id = ? AND pst_time > ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY pst_time DESC")
            .bind(account_id.to_string())
            .bind(from)
            .bind(to)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id_and_account_id(&self, id: Uuid, account_id: Uuid) -> Result<Option<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE id = ? AND account_id = ?")
            .bind(id.to_string())
            .bind(account_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }
    
    async fn find_by_base_line_and_pst_time_less_than_equal(&self, base_line: &str, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE base_line = ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(base_line)
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_account_and_pst_time_less_than_equal(&self, account_id: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        sqlx::query_as("SELECT * FROM posting_line WHERE account_id = ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(account_id.to_string())
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
