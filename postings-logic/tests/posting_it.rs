use std::sync::Arc;
use sqlx::PgPool;
use postings_logic::services::posting_service::PostingServiceImpl;
use postings_api::service::posting_service::PostingService;
use postings_db_postgres::repositories::posting_repository::PostgresPostingRepository;
use postings_logic::services::shared_service::SharedService;
use postings_api::domain::posting::Posting;
use postings_api::domain::ledger::Ledger;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::domain::named::Named;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use postings_api::domain::posting_line::PostingLine;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::domain::balance_side::BalanceSide;
use postings_api::domain::account_category::AccountCategory;
use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;

async fn setup_ledger(pool: &PgPool) -> anyhow::Result<Ledger> {
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

    let ledger = Ledger {
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

    sqlx::query("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(ledger.named.id.to_string())
        .bind(&ledger.named.name)
        .bind(&ledger.coa.named.id.to_string())
        .bind(ledger.named.created)
        .bind(&ledger.named.user_details)
        .bind(&ledger.named.short_desc)
        .bind(&ledger.named.long_desc)
        .execute(pool)
        .await?;
        
    Ok(ledger)
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_posting(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let ledger = setup_ledger(&pool).await?;
    let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
    let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
    let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
    let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
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
    let service = PostingServiceImpl::new(shared_service);

    let debit_account = LedgerAccount {
        named: Named { id: Uuid::new_v4(), name: "Debit Account".to_string(), created: chrono::Utc::now(), user_details: "test".to_string(), short_desc: None, long_desc: None },
        ledger: ledger.clone(),
        parent: None,
        coa: ledger.coa.clone(),
        balance_side: BalanceSide::Dr,
        category: AccountCategory::AS,
    };
    let credit_account = LedgerAccount {
        named: Named { id: Uuid::new_v4(), name: "Credit Account".to_string(), created: chrono::Utc::now(), user_details: "test".to_string(), short_desc: None, long_desc: None },
        ledger: ledger.clone(),
        parent: None,
        coa: ledger.coa.clone(),
        balance_side: BalanceSide::Cr,
        category: AccountCategory::LI,
    };

    let posting_bo = Posting {
        id: Uuid::new_v4(),
        record_user: "test_user".to_string(),
        record_time: chrono::Utc::now(),
        opr_id: "test_opr".to_string(),
        opr_time: chrono::Utc::now(),
        opr_type: "test_type".to_string(),
        opr_details: "test_details".to_string(),
        opr_src: None,
        pst_time: chrono::Utc::now(),
        pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
        pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
        ledger,
        val_time: None,
        lines: vec![
            PostingLine {
                id: Uuid::new_v4(),
                account: debit_account,
                debit_amount: BigDecimal::from(100),
                credit_amount: BigDecimal::from(0),
                details: "debit".to_string(),
                src_account: None,
                base_line: None,
                sub_opr_src_id: None,
                record_time: chrono::Utc::now(),
                opr_id: "test_opr".to_string(),
                opr_src: None,
                pst_time: chrono::Utc::now(),
                pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                hash: "hash".to_string(),
                additional_information: None,
                discarded_time: None,
            },
            PostingLine {
                id: Uuid::new_v4(),
                account: credit_account,
                debit_amount: BigDecimal::from(0),
                credit_amount: BigDecimal::from(100),
                details: "credit".to_string(),
                src_account: None,
                base_line: None,
                sub_opr_src_id: None,
                record_time: chrono::Utc::now(),
                opr_id: "test_opr".to_string(),
                opr_src: None,
                pst_time: chrono::Utc::now(),
                pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                hash: "hash".to_string(),
                additional_information: None,
                discarded_time: None,
            }
        ],
        discarded_id: None,
        discarded_time: None,
        discarding_id: None,
        hash_record: Default::default(),
    };

    // Act
    let result = service.new_posting(posting_bo).await?;

    // Assert
    assert_eq!(result.opr_id, "test_opr");
    
    Ok(())
}