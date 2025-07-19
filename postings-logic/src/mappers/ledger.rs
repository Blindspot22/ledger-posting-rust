use postings_api::domain::ledger::Ledger as LedgerBO;
use postings_db::models::ledger::Ledger as LedgerModel;

pub struct LedgerMapper;

impl LedgerMapper {
    pub fn to_bo(model: LedgerModel, coa_bo: postings_api::domain::chart_of_account::ChartOfAccount) -> LedgerBO {
        LedgerBO {
            id: model.id,
            coa: coa_bo,
        }
    }

    pub fn to_model(bo: LedgerBO) -> LedgerModel {
        LedgerModel {
            id: bo.id,
            coa_id: bo.coa.id,
        }
    }
}
