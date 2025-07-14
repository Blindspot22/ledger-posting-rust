use std::sync::Arc;
use bigdecimal::BigDecimal;
use chrono::Utc;
use sqlx::{PgPool, Type};
use uuid::Uuid;

use postings_api::domain::{
    account_category::AccountCategory, balance_side::BalanceSide, chart_of_account::ChartOfAccount,
    ledger::Ledger, ledger_account::LedgerAccount, named::Named,
};
use postings_api::service::account_stmt_service::AccountStmtService;
use postings_db::models::posting_line::PostingLine as PostingLineModel;
use postings_db_postgres::repositories::{
    account_stmt_repository::PostgresAccountStmtRepository,
    chart_of_account_repository::PostgresChartOfAccountRepository,
    ledger_account_repository::PostgresLedgerAccountRepository,
    ledger_repository::PostgresLedgerRepository,
    posting_line_repository::PostgresPostingLineRepository,
    posting_repository::PostgresPostingRepository,
    posting_trace_repository::PostgresPostingTraceRepository,
};
use postings_logic::services::{
    account_stmt_service::AccountStmtServiceImpl, shared_service::SharedService,
};

#[derive(Type)]
#[sqlx(type_name = "balance_side")]
pub enum TestBalanceSide {
    Dr,
    Cr,
    DrCr,
}

#[derive(Type)]
#[sqlx(type_name = "account_category")]
pub enum TestAccountCategory {
    RE,
    EX,
    AS,
    LI,
    EQ,
    NOOP,
    NORE,
    NOEX,
}

async fn setup_test_data(pool: &PgPool) -> anyhow::Result<(LedgerAccount, Ledger)> {
    let coa = ChartOfAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test COA".to_string(),
            created: Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: Some("Short desc".to_string()),
            long_desc: Some("Long desc".to_string()),
        },
    };
    sqlx::query("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(coa.named.id)
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
            created: Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: Some("Short desc".to_string()),
            long_desc: Some("Long desc".to_string()),
        },
        coa: coa.clone(),
    };
    sqlx::query("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(ledger.named.id)
        .bind(&ledger.named.name)
        .bind(ledger.coa.named.id)
        .bind(ledger.named.created)
        .bind(&ledger.named.user_details)
        .bind(&ledger.named.short_desc)
        .bind(&ledger.named.long_desc)
        .execute(pool)
        .await?;

    let ledger_account = LedgerAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test Account".to_string(),
            created: Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: None,
            long_desc: None,
        },
        ledger: ledger.clone(),
        parent: None,
        coa,
        balance_side: BalanceSide::Dr,
        category: AccountCategory::AS,
    };
    sqlx::query("INSERT INTO ledger_account (id, name, ledger_id, coa_id, balance_side, category, created, user_details) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(ledger_account.named.id)
        .bind(&ledger_account.named.name)
        .bind(ledger_account.ledger.named.id)
        .bind(ledger_account.coa.named.id)
        .bind(match ledger_account.balance_side {
            BalanceSide::Dr => TestBalanceSide::Dr,
            BalanceSide::Cr => TestBalanceSide::Cr,
            BalanceSide::DrCr => TestBalanceSide::DrCr,
        })
        .bind(match ledger_account.category {
            AccountCategory::RE => TestAccountCategory::RE,
            AccountCategory::EX => TestAccountCategory::EX,
            AccountCategory::AS => TestAccountCategory::AS,
            AccountCategory::LI => TestAccountCategory::LI,
            AccountCategory::EQ => TestAccountCategory::EQ,
            AccountCategory::NOOP => TestAccountCategory::NOOP,
            AccountCategory::NORE => TestAccountCategory::NORE,
            AccountCategory::NOEX => TestAccountCategory::NOEX,
        })
        .bind(ledger_account.named.created)
        .bind(&ledger_account.named.user_details)
        .execute(pool)
        .await?;

    Ok((ledger_account, ledger))
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_read_stmt(pool: PgPool) -> anyhow::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();
    dotenvy::dotenv().ok();
    // Arrange
    let (ledger_account, _ledger) = setup_test_data(&pool).await?;

    let now = Utc::now();

    let line1 = PostingLineModel {
        id: Uuid::new_v4(),
        account_id: ledger_account.named.id,
        debit_amount: BigDecimal::from(100),
        credit_amount: BigDecimal::from(0),
        details: Some("{}".to_string()),
        src_account: None,
        base_line: None,
        sub_opr_src_id: None,
        record_time: now,
        opr_id: "test_opr".to_string(),
        opr_src: None,
        pst_time: now,
        pst_type: postings_db::models::posting_type::PostingType::BusiTx,
        pst_status: postings_db::models::posting_status::PostingStatus::Posted,
        hash: "hash1".to_string(),
        discarded_time: None,
    };
    let line2 = PostingLineModel {
        id: Uuid::new_v4(),
        account_id: ledger_account.named.id,
        debit_amount: BigDecimal::from(50),
        credit_amount: BigDecimal::from(0),
        details: Some("{}".to_string()),
        src_account: None,
        base_line: None,
        sub_opr_src_id: None,
        record_time: now,
        opr_id: "test_opr".to_string(),
        opr_src: None,
        pst_time: now,
        pst_type: postings_db::models::posting_type::PostingType::BusiTx,
        pst_status: postings_db::models::posting_status::PostingStatus::Posted,
        hash: "hash2".to_string(),
        discarded_time: None,
    };
    sqlx::query("INSERT INTO posting_line (id, account_id, debit_amount, credit_amount, details, record_time, opr_id, pst_time, pst_type, pst_status, hash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
        .bind(line1.id)
        .bind(line1.account_id)
        .bind(&line1.debit_amount)
        .bind(&line1.credit_amount)
        .bind(&line1.details)
        .bind(line1.record_time)
        .bind(&line1.opr_id)
        .bind(line1.pst_time)
        .bind(&line1.pst_type)
        .bind(&line1.pst_status)
        .bind(&line1.hash)
        .execute(&pool)
        .await?;
    sqlx::query("INSERT INTO posting_line (id, account_id, debit_amount, credit_amount, details, record_time, opr_id, pst_time, pst_type, pst_status, hash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
        .bind(line2.id)
        .bind(line2.account_id)
        .bind(&line2.debit_amount)
        .bind(&line2.credit_amount)
        .bind(&line2.details)
        .bind(line2.record_time)
        .bind(&line2.opr_id)
        .bind(line2.pst_time)
        .bind(&line2.pst_type)
        .bind(&line2.pst_status)
        .bind(&line2.hash)
        .execute(&pool)
        .await?;

    let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
    let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
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
    let service = AccountStmtServiceImpl::new(shared_service);

    // Act
    let result = service.read_stmt(ledger_account, Utc::now()).await?;

    // Assert
    assert_eq!(result.total_debit, BigDecimal::from(150));
    assert_eq!(result.total_credit, BigDecimal::from(0));

    Ok(())
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_create_stmt_no_closed_statement(pool: PgPool) -> anyhow::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();
    dotenvy::dotenv().ok();
    // Arrange
    let (ledger_account, _ledger) = setup_test_data(&pool).await?;
    let service = create_service(pool.clone());

    // Act
    let result = service.create_stmt(ledger_account.clone(), Utc::now()).await?;

    // Assert
    let stmt_model = sqlx::query_as::<_, postings_db::models::account_stmt::AccountStmt>("SELECT * FROM account_stmt WHERE id = $1")
        .bind(result.financial_stmt.id)
        .fetch_one(&pool)
        .await?;
    
    assert_eq!(stmt_model.id, result.financial_stmt.id);
    assert_eq!(stmt_model.account_id, ledger_account.named.id);
    assert_eq!(stmt_model.stmt_status, postings_db::models::stmt_status::StmtStatus::Simulated);

    Ok(())
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_close_stmt(pool: PgPool) -> anyhow::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();
    dotenvy::dotenv().ok();
    // Arrange
    let (ledger_account, _ledger) = setup_test_data(&pool).await?;
    let service = create_service(pool.clone());
    let created_stmt = service.create_stmt(ledger_account.clone(), Utc::now()).await?;

    // Act
    let closed_stmt = service.close_stmt(created_stmt).await?;

    // Assert
    assert_eq!(closed_stmt.financial_stmt.stmt_status, postings_api::domain::stmt_status::StmtStatus::CLOSED);
    assert!(closed_stmt.financial_stmt.posting.is_some());

    let stmt_model = sqlx::query_as::<_, postings_db::models::account_stmt::AccountStmt>("SELECT * FROM account_stmt WHERE id = $1")
        .bind(closed_stmt.financial_stmt.id)
        .fetch_one(&pool)
        .await?;
    
    assert_eq!(stmt_model.stmt_status, postings_db::models::stmt_status::StmtStatus::Closed);
    assert_eq!(stmt_model.posting_id, Some(closed_stmt.financial_stmt.posting.unwrap().id));

    Ok(())
}

fn create_service(pool: PgPool) -> AccountStmtServiceImpl {
    let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
    let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
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
    AccountStmtServiceImpl::new(shared_service)
}
