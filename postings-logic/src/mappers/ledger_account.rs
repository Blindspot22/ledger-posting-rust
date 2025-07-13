use postings_api::domain::ledger_account::LedgerAccount as LedgerAccountBO;
use postings_db::models::ledger_account::LedgerAccount as LedgerAccountModel;
use postings_api::domain::named::Named;
use uuid::Uuid;

pub struct LedgerAccountMapper;

impl LedgerAccountMapper {
    pub fn to_bo(model: LedgerAccountModel, ledger_bo: postings_api::domain::ledger::Ledger, coa_bo: postings_api::domain::chart_of_account::ChartOfAccount, parent_bo: Option<Box<LedgerAccountBO>>) -> LedgerAccountBO {
        LedgerAccountBO {
            named: Named {
                id: Uuid::parse_str(&model.id).unwrap(),
                name: model.name,
                created: model.created,
                user_details: model.user_details,
                short_desc: model.short_desc,
                long_desc: model.long_desc,
            },
            ledger: ledger_bo,
            parent: parent_bo,
            coa: coa_bo,
            balance_side: match model.balance_side {
                postings_db::models::balance_side::BalanceSide::Dr => postings_api::domain::balance_side::BalanceSide::Dr,
                postings_db::models::balance_side::BalanceSide::Cr => postings_api::domain::balance_side::BalanceSide::Cr,
                postings_db::models::balance_side::BalanceSide::DrCr => postings_api::domain::balance_side::BalanceSide::DrCr,
            },
            category: match model.category {
                postings_db::models::account_category::AccountCategory::RE => postings_api::domain::account_category::AccountCategory::RE,
                postings_db::models::account_category::AccountCategory::EX => postings_api::domain::account_category::AccountCategory::EX,
                postings_db::models::account_category::AccountCategory::AS => postings_api::domain::account_category::AccountCategory::AS,
                postings_db::models::account_category::AccountCategory::LI => postings_api::domain::account_category::AccountCategory::LI,
                postings_db::models::account_category::AccountCategory::EQ => postings_api::domain::account_category::AccountCategory::EQ,
                postings_db::models::account_category::AccountCategory::NOOP => postings_api::domain::account_category::AccountCategory::NOOP,
                postings_db::models::account_category::AccountCategory::NORE => postings_api::domain::account_category::AccountCategory::NORE,
                postings_db::models::account_category::AccountCategory::NOEX => postings_api::domain::account_category::AccountCategory::NOEX,
            },
        }
    }

    pub fn to_model(bo: LedgerAccountBO) -> LedgerAccountModel {
        LedgerAccountModel {
            id: bo.named.id.to_string(),
            name: bo.named.name,
            ledger_id: bo.ledger.named.id.to_string(),
            parent_id: bo.parent.map(|p| p.named.id.to_string()),
            coa_id: bo.coa.named.id.to_string(),
            balance_side: match bo.balance_side {
                postings_api::domain::balance_side::BalanceSide::Dr => postings_db::models::balance_side::BalanceSide::Dr,
                postings_api::domain::balance_side::BalanceSide::Cr => postings_db::models::balance_side::BalanceSide::Cr,
                postings_api::domain::balance_side::BalanceSide::DrCr => postings_db::models::balance_side::BalanceSide::DrCr,
            },
            category: match bo.category {
                postings_api::domain::account_category::AccountCategory::RE => postings_db::models::account_category::AccountCategory::RE,
                postings_api::domain::account_category::AccountCategory::EX => postings_db::models::account_category::AccountCategory::EX,
                postings_api::domain::account_category::AccountCategory::AS => postings_db::models::account_category::AccountCategory::AS,
                postings_api::domain::account_category::AccountCategory::LI => postings_db::models::account_category::AccountCategory::LI,
                postings_api::domain::account_category::AccountCategory::EQ => postings_db::models::account_category::AccountCategory::EQ,
                postings_api::domain::account_category::AccountCategory::NOOP => postings_db::models::account_category::AccountCategory::NOOP,
                postings_api::domain::account_category::AccountCategory::NORE => postings_db::models::account_category::AccountCategory::NORE,
                postings_api::domain::account_category::AccountCategory::NOEX => postings_db::models::account_category::AccountCategory::NOEX,
            },
            created: bo.named.created,
            user_details: bo.named.user_details,
            short_desc: bo.named.short_desc,
            long_desc: bo.named.long_desc,
        }
    }
}
