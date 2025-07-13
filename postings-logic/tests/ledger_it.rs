use std::sync::Arc;
use serde::Deserialize;
use sqlx::PgPool;
use postings_logic::services::ledger_service::LedgerServiceImpl;
use postings_api::service::ledger_service::LedgerService;
use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
use postings_db_postgres::repositories::posting_repository::PostgresPostingRepository;
use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;
use postings_logic::services::shared_service::SharedService;
use postings_api::domain::ledger::Ledger;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::domain::named::Named;
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChartOfAccountSeed {
    id: String,
    user_details: String,
    created: chrono::NaiveDateTime,
    name: String,
    short_desc: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct TestDataSet {
    charts_of_account: Vec<ChartOfAccountSeed>,
}

async fn setup_coa(pool: &PgPool) -> anyhow::Result<ChartOfAccount> {
    let coa = ChartOfAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test COA".to_string(),
            created: chrono::Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: Some("Short desc".to_string()),
            long_desc: Some("Long desc".to_string()),
        }
    };
    
    sqlx::query("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(coa.named.id.to_string())
        .bind(&coa.named.name)
        .bind(coa.named.created)
        .bind(&coa.named.user_details)
        .bind(&coa.named.short_desc)
        .bind(&coa.named.long_desc)
        .execute(pool)
        .await?;
        
    Ok(coa)
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_ledger(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
    let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
    let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
    let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
    let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
    let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
    let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

    let shared_service = SharedService::new(
        coa_repo,
        ledger_repo,
        ledger_account_repo,
        posting_repo,
        stmt_repo,
        line_repo,
        trace_repo,
    );
    let service = LedgerServiceImpl::new(shared_service);

    let ledger_bo = Ledger {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test Ledger".to_string(),
            created: chrono::Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: Some("Short desc".to_string()),
            long_desc: Some("Long desc".to_string()),
        },
        coa,
    };

    // Act
    let result = service.new_ledger(ledger_bo).await?;

    // Assert
    assert_eq!(result.named.name, "Test Ledger");
    
    Ok(())
}
