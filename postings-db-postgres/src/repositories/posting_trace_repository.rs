use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::posting_trace_repository::PostingTraceRepository;
use postings_db::models::posting_trace::PostingTrace;
use postings_db::DbError;
use uuid::Uuid;

pub struct PostgresPostingTraceRepository {
    pool: PgPool,
}

impl PostgresPostingTraceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostingTraceRepository for PostgresPostingTraceRepository {
    async fn save(&self, trace: PostingTrace) -> Result<PostingTrace, DbError> {
        sqlx::query_as("INSERT INTO posting_trace (id, tgt_pst_id, src_pst_time, src_pst_id, src_opr_id, account_id, debit_amount, credit_amount, src_pst_hash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *")
            .bind(trace.id)
            .bind(trace.tgt_pst_id)
            .bind(trace.src_pst_time)
            .bind(trace.src_pst_id)
            .bind(trace.src_opr_id)
            .bind(trace.account_id)
            .bind(trace.debit_amount)
            .bind(trace.credit_amount)
            .bind(trace.src_pst_hash)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingTrace>, DbError> {
        sqlx::query_as("SELECT * FROM posting_trace WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
