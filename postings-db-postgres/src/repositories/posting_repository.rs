use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::posting_repository::PostingRepository;
use postings_db::models::posting::Posting;
use postings_db::DbError;
use uuid::Uuid;

pub struct PostgresPostingRepository {
    pool: PgPool,
}

impl PostgresPostingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostingRepository for PostgresPostingRepository {
    async fn find_by_opr_id_and_discarding_id_is_null(&self, opr_id: &[u8]) -> Result<Option<Posting>, DbError> {
        sqlx::query_as("SELECT * FROM posting WHERE opr_id = $1 AND discarding_id IS NULL")
            .bind(opr_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_opr_id(&self, opr_id: &[u8]) -> Result<Vec<Posting>, DbError> {
        sqlx::query_as("SELECT * FROM posting WHERE opr_id = $1")
            .bind(opr_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_first_by_ledger_order_by_record_time_desc(&self, ledger_id: Uuid) -> Result<Option<Posting>, DbError> {
        sqlx::query_as("SELECT * FROM posting WHERE ledger_id = $1 ORDER BY record_time DESC LIMIT 1")
            .bind(ledger_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, posting: &Posting) -> Result<(), DbError> {
        sqlx::query("INSERT INTO posting (id, record_user, record_time, opr_id, opr_time, opr_type, opr_details, opr_src, pst_time, pst_type, pst_status, ledger_id, val_time, discarded_id, discarded_time, discarding_id, antecedent_id, antecedent_hash, hash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)")
            .bind(posting.id)
            .bind(posting.record_user)
            .bind(posting.record_time)
            .bind(posting.opr_id)
            .bind(posting.opr_time)
            .bind(posting.opr_type)
            .bind(posting.opr_details)
            .bind(posting.opr_src)
            .bind(posting.pst_time)
            .bind(&posting.pst_type)
            .bind(&posting.pst_status)
            .bind(posting.ledger_id)
            .bind(posting.val_time)
            .bind(posting.discarded_id)
            .bind(posting.discarded_time)
            .bind(posting.discarding_id)
            .bind(posting.antecedent_id)
            .bind(posting.antecedent_hash)
            .bind(posting.hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Posting>, DbError> {
        sqlx::query_as("SELECT * FROM posting WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
