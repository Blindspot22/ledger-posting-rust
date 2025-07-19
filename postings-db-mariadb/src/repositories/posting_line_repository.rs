use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::posting_line_repository::PostingLineRepository;
use postings_db::models::posting_line::PostingLine;
use postings_db::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::posting_line::PostingLineDb;

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
    async fn save(&self, posting_line: PostingLine) -> Result<PostingLine, DbError> {
        let db_model = PostingLineDb::from(posting_line.clone());
        
        sqlx::query("INSERT INTO posting_line (id, account_id, debit_amount, credit_amount, details, src_account, base_line, sub_opr_src_id, record_time, opr_id, opr_src, pst_time, pst_type, pst_status, hash, discarded_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&db_model.id)
            .bind(&db_model.account_id)
            .bind(&db_model.debit_amount)
            .bind(&db_model.credit_amount)
            .bind(&db_model.details)
            .bind(&db_model.src_account)
            .bind(&db_model.base_line)
            .bind(&db_model.sub_opr_src_id)
            .bind(db_model.record_time)
            .bind(&db_model.opr_id)
            .bind(&db_model.opr_src)
            .bind(db_model.pst_time)
            .bind(&db_model.pst_type)
            .bind(&db_model.pst_status)
            .bind(&db_model.hash)
            .bind(db_model.discarded_time)
            .execute(&self.pool)
            .await
            .map_err(DbError::from)?;
            
        // Return the saved posting line (use original posting_line since save succeeded)
        Ok(posting_line)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingLine>, DbError> {
        let posting_line_db = sqlx::query_as::<_, PostingLineDb>("SELECT * FROM posting_line WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_line_db.map(Into::into))
    }

    async fn find_by_account_and_pst_time_between(&self, account_id: Uuid, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        let posting_lines_db = sqlx::query_as::<_, PostingLineDb>("SELECT * FROM posting_line WHERE account_id = ? AND pst_time > ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY pst_time DESC")
            .bind(account_id.to_string())
            .bind(from)
            .bind(to)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_lines_db.into_iter().map(Into::into).collect())
    }

    async fn find_by_id_and_account_id(&self, id: Uuid, account_id: Uuid) -> Result<Option<PostingLine>, DbError> {
        let posting_line_db = sqlx::query_as::<_, PostingLineDb>("SELECT * FROM posting_line WHERE id = ? AND account_id = ?")
            .bind(id.to_string())
            .bind(account_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_line_db.map(Into::into))
    }
    
    async fn find_by_base_line_and_pst_time_less_than_equal(&self, base_line: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        let posting_lines_db = sqlx::query_as::<_, PostingLineDb>("SELECT * FROM posting_line WHERE base_line = ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(base_line.to_string())
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_lines_db.into_iter().map(Into::into).collect())
    }

    async fn find_by_account_and_pst_time_less_than_equal(&self, account_id: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError> {
        let posting_lines_db = sqlx::query_as::<_, PostingLineDb>("SELECT * FROM posting_line WHERE account_id = ? AND pst_time <= ? AND discarded_time IS NULL ORDER BY record_time DESC")
            .bind(account_id.to_string())
            .bind(ref_time)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_lines_db.into_iter().map(Into::into).collect())
    }
}
