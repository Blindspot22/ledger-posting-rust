use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::posting_repository::PostingRepository;
use postings_db::models::posting::Posting;
use postings_db::DbError;
use uuid::Uuid;

pub struct MariaDbPostingRepository {
    pool: MySqlPool,
}

impl MariaDbPostingRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

use crate::models::posting::PostingDb;

#[async_trait]
impl PostingRepository for MariaDbPostingRepository {
    async fn find_by_opr_id_and_discarding_id_is_null(&self, opr_id: &[u8]) -> Result<Option<Posting>, DbError> {
        let posting_db = sqlx::query_as::<_, PostingDb>("SELECT * FROM posting WHERE opr_id = ? AND discarding_id IS NULL")
            .bind(opr_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_db.map(Into::into))
    }

    async fn find_by_opr_id(&self, opr_id: &[u8]) -> Result<Vec<Posting>, DbError> {
        let postings_db = sqlx::query_as::<_, PostingDb>("SELECT * FROM posting WHERE opr_id = ?")
            .bind(opr_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(postings_db.into_iter().map(Into::into).collect())
    }

    async fn find_first_by_ledger_order_by_record_time_desc(&self, ledger_id: Uuid) -> Result<Option<Posting>, DbError> {
        let posting_db = sqlx::query_as::<_, PostingDb>("SELECT * FROM posting WHERE ledger_id = ? ORDER BY record_time DESC LIMIT 1")
            .bind(ledger_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_db.map(Into::into))
    }

    async fn save(&self, posting: &Posting) -> Result<(), DbError> {
        sqlx::query("INSERT INTO posting (id, record_user, record_time, opr_id, opr_time, opr_type, opr_details, opr_src, pst_time, pst_type, pst_status, ledger_id, val_time, discarded_id, discarded_time, discarding_id, antecedent_id, antecedent_hash, hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(posting.id.to_string())
            .bind(posting.record_user.as_ref())
            .bind(posting.record_time)
            .bind(posting.opr_id.as_ref())
            .bind(posting.opr_time)
            .bind(posting.opr_type.as_ref())
            .bind(posting.opr_details.as_ref().map(|v| v.as_ref()))
            .bind(posting.opr_src.as_ref().map(|v| v.as_ref()))
            .bind(posting.pst_time)
            .bind(&posting.pst_type)
            .bind(&posting.pst_status)
            .bind(posting.ledger_id.to_string())
            .bind(posting.val_time)
            .bind(posting.discarded_id.map(|u| u.to_string()))
            .bind(posting.discarded_time)
            .bind(posting.discarding_id.map(|u| u.to_string()))
            .bind(posting.antecedent_id.map(|u| u.to_string()))
            .bind(posting.antecedent_hash.as_ref().map(|v| v.as_ref()))
            .bind(posting.hash.as_ref().map(|v| v.as_ref()))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Posting>, DbError> {
        let posting_db = sqlx::query_as::<_, PostingDb>("SELECT * FROM posting WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(posting_db.map(Into::into))
    }
}