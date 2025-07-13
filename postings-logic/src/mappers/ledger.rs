use postings_api::domain::ledger::Ledger as LedgerBO;
use postings_db::models::ledger::Ledger as LedgerModel;
use postings_api::domain::named::Named;
use uuid::Uuid;

pub struct LedgerMapper;

impl LedgerMapper {
    pub fn to_bo(model: LedgerModel, coa_bo: postings_api::domain::chart_of_account::ChartOfAccount) -> LedgerBO {
        LedgerBO {
            named: Named {
                id: Uuid::parse_str(&model.id).unwrap(),
                name: model.name,
                created: model.created,
                user_details: model.user_details,
                short_desc: model.short_desc,
                long_desc: model.long_desc,
            },
            coa: coa_bo,
        }
    }

    pub fn to_model(bo: LedgerBO) -> LedgerModel {
        LedgerModel {
            id: bo.named.id.to_string(),
            name: bo.named.name,
            coa_id: bo.coa.named.id.to_string(),
            created: bo.named.created,
            user_details: bo.named.user_details,
            short_desc: bo.named.short_desc,
            long_desc: bo.named.long_desc,
        }
    }
}
