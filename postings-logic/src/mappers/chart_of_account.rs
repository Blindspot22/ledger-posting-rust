use postings_api::domain::chart_of_account::ChartOfAccount as ChartOfAccountBO;
use postings_db::models::chart_of_account::ChartOfAccount as ChartOfAccountModel;
use postings_api::domain::named::Named;
use uuid::Uuid;

pub struct ChartOfAccountMapper;

impl ChartOfAccountMapper {
    pub fn to_bo(model: ChartOfAccountModel) -> ChartOfAccountBO {
        ChartOfAccountBO {
            named: Named {
                id: Uuid::parse_str(&model.id).unwrap(),
                name: model.name,
                created: model.created,
                user_details: model.user_details,
                short_desc: model.short_desc,
                long_desc: model.long_desc,
            }
        }
    }

    pub fn to_model(bo: ChartOfAccountBO) -> ChartOfAccountModel {
        ChartOfAccountModel {
            id: bo.named.id.to_string(),
            name: bo.named.name,
            created: bo.named.created,
            user_details: bo.named.user_details,
            short_desc: bo.named.short_desc,
            long_desc: bo.named.long_desc,
        }
    }
}
