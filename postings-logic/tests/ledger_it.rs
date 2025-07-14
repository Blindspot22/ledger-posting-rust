use std::sync::Arc;
use sqlx::{PgPool, Type};
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
use postings_api::domain::{
    account_category::AccountCategory, balance_side::BalanceSide, ledger_account::LedgerAccount,
};
use postings_api::ServiceError;

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
        .bind(coa.named.id)
        .bind(&coa.named.name)
        .bind(coa.named.created)
        .bind(&coa.named.user_details)
        .bind(&coa.named.short_desc)
        .bind(&coa.named.long_desc)
        .execute(pool)
        .await?;
        
    Ok(coa)
}

async fn setup_ledger(pool: &PgPool, coa: &ChartOfAccount) -> anyhow::Result<Ledger> {
    let ledger = Ledger {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test Ledger".to_string(),
            created: chrono::Utc::now(),
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
    Ok(ledger)
}

async fn setup_ledger_account(pool: &PgPool, ledger: &Ledger, name: &str, category: AccountCategory, balance_side: BalanceSide, parent: Option<&LedgerAccount>) -> anyhow::Result<LedgerAccount> {
    let ledger_account = LedgerAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: name.to_string(),
            created: chrono::Utc::now(),
            user_details: "test_user".to_string(),
            short_desc: None,
            long_desc: None,
        },
        ledger: ledger.clone(),
        parent: parent.map(|p| Box::new(p.clone())),
        coa: ledger.coa.clone(),
        balance_side,
        category,
    };
    sqlx::query("INSERT INTO ledger_account (id, name, ledger_id, parent_id, coa_id, balance_side, category, created, user_details) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(ledger_account.named.id)
        .bind(&ledger_account.named.name)
        .bind(ledger_account.ledger.named.id)
        .bind(parent.map(|p| p.named.id))
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

    Ok(ledger_account)
}

fn create_service(pool: PgPool) -> LedgerServiceImpl {
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
    LedgerServiceImpl::new(shared_service)
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_ledger(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let service = create_service(pool);

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

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_ledger_account_with_parent_inheritance(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let ledger = setup_ledger(&pool, &coa).await?;
    let service = create_service(pool.clone());
    let parent_account = setup_ledger_account(&pool, &ledger, "Parent", AccountCategory::AS, BalanceSide::Dr, None).await?;

    let child_account_bo = LedgerAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Child Account".to_string(),
            created: chrono::Utc::now(),
            user_details: "".to_string(),
            short_desc: None,
            long_desc: None,
        },
        ledger: ledger.clone(),
        parent: Some(Box::new(parent_account.clone())),
        coa: coa.clone(),
        balance_side: BalanceSide::DrCr, // Should be inherited
        category: AccountCategory::NOOP, // Should be inherited
    };

    // Act
    let result = service.new_ledger_account(child_account_bo, "test_user").await?;

    // Assert
    assert_eq!(result.category, parent_account.category);
    assert_eq!(result.balance_side, parent_account.balance_side);
    assert_eq!(result.parent.unwrap().named.id, parent_account.named.id);

    Ok(())
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_ledger_account_no_parent_no_category_fails(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let ledger = setup_ledger(&pool, &coa).await?;
    let service = create_service(pool);

    let account_bo = LedgerAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test Account".to_string(),
            created: chrono::Utc::now(),
            user_details: "".to_string(),
            short_desc: None,
            long_desc: None,
        },
        ledger: ledger.clone(),
        parent: None,
        coa: coa.clone(),
        balance_side: BalanceSide::DrCr,
        category: AccountCategory::NOOP, // No category specified and no parent
    };

    // Act
    let result = service.new_ledger_account(account_bo, "test_user").await;

    // Assert
    assert!(matches!(result, Err(ServiceError::NoCategory)));

    Ok(())
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_new_ledger_account_no_parent_uses_category_default_balance_side(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let ledger = setup_ledger(&pool, &coa).await?;
    let service = create_service(pool);

    let account_bo = LedgerAccount {
        named: Named {
            id: Uuid::new_v4(),
            name: "Test Account".to_string(),
            created: chrono::Utc::now(),
            user_details: "".to_string(),
            short_desc: None,
            long_desc: None,
        },
        ledger: ledger.clone(),
        parent: None,
        coa: coa.clone(),
        balance_side: BalanceSide::DrCr, // Should use default
        category: AccountCategory::AS, // Asset -> Dr
    };

    // Act
    let result = service.new_ledger_account(account_bo, "test_user").await?;

    // Assert
    assert_eq!(result.balance_side, AccountCategory::AS.default_bs());
    assert_eq!(result.balance_side, BalanceSide::Dr);

    Ok(())
}

#[sqlx::test(migrations = "../postings-db-postgres/migrations")]
async fn test_find_ledger_account_by_id_with_parent_hierarchy(pool: PgPool) -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Arrange
    let coa = setup_coa(&pool).await?;
    let ledger = setup_ledger(&pool, &coa).await?;
    let service = create_service(pool.clone());

    let grandparent = setup_ledger_account(&pool, &ledger, "Grandparent", AccountCategory::AS, BalanceSide::Dr, None).await?;
    let parent = setup_ledger_account(&pool, &ledger, "Parent", AccountCategory::AS, BalanceSide::Dr, Some(&grandparent)).await?;
    let child = setup_ledger_account(&pool, &ledger, "Child", AccountCategory::AS, BalanceSide::Dr, Some(&parent)).await?;

    // Act
    let result = service.find_ledger_account_by_id(child.named.id).await?;

    // Assert
    assert_eq!(result.named.id, child.named.id);
    let p = result.parent.unwrap();
    assert_eq!(p.named.id, parent.named.id);
    let gp = p.parent.unwrap();
    assert_eq!(gp.named.id, grandparent.named.id);
    assert!(gp.parent.is_none());

    Ok(())
}
