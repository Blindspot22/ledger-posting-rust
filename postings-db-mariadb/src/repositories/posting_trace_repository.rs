use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::posting_trace_repository::PostingTraceRepository;
use postings_db::models::posting_trace::PostingTrace;
use postings_db::DbError;
use uuid::Uuid;
use crate::models::posting_trace::PostingTraceDb;

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
        let trace_db = PostingTraceDb::from(trace.clone());
        sqlx::query("INSERT INTO posting_trace (id, tgt_pst_id, src_pst_time, src_pst_id, src_opr_id, account_id, debit_amount, credit_amount, src_pst_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(trace_db.id.to_string())
            .bind(trace_db.tgt_pst_id.to_string())
            .bind(trace_db.src_pst_time)
            .bind(trace_db.src_pst_id.to_string())
            .bind(&trace_db.src_opr_id)
            .bind(trace_db.account_id.to_string())
            .bind(&trace_db.debit_amount)
            .bind(&trace_db.credit_amount)
            .bind(&trace_db.src_pst_hash)
            .execute(&self.pool)
            .await?;
        Ok(trace)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingTrace>, DbError> {
        let trace_db = sqlx::query_as::<_, PostingTraceDb>("SELECT * FROM posting_trace WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        Ok(trace_db.map(Into::into))
    }
}