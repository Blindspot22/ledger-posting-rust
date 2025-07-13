use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::posting_trace_repository::PostingTraceRepository;
use postings_db::models::posting_trace::PostingTrace;
use postings_db::DbError;

pub struct MariaDbPostingTraceRepository {
    pool: MySqlPool,
}

impl MariaDbPostingTraceRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostingTraceRepository for MariaDbPostingTraceRepository {
    async fn save(&self, trace: PostingTrace) -> Result<PostingTrace, DbError> {
        sqlx::query("INSERT INTO posting_trace (id, tgt_pst_id, src_pst_time, src_pst_id, src_opr_id, account_id, debit_amount, credit_amount, src_pst_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&trace.id)
            .bind(&trace.tgt_pst_id)
            .bind(trace.src_pst_time)
            .bind(&trace.src_pst_id)
            .bind(&trace.src_opr_id)
            .bind(&trace.account_id)
            .bind(&trace.debit_amount)
            .bind(&trace.credit_amount)
            .bind(&trace.src_pst_hash)
            .execute(&self.pool)
            .await?;
        Ok(trace)
    }
}