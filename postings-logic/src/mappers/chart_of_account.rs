use postings_api::domain::chart_of_account::ChartOfAccount as ChartOfAccountBO;
use postings_db::models::chart_of_account::ChartOfAccount as ChartOfAccountModel;

pub struct ChartOfAccountMapper;

impl ChartOfAccountMapper {
    pub fn to_bo(model: ChartOfAccountModel) -> ChartOfAccountBO {
        ChartOfAccountBO {
            id: model.id,
        }
    }

    pub fn to_model(bo: ChartOfAccountBO) -> ChartOfAccountModel {
        ChartOfAccountModel {
            id: bo.id,
        }
    }
}
