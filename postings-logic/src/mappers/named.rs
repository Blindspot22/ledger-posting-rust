use postings_api::domain::named::Named as NamedBO;
use postings_db::models::named::Named as NamedModel;

pub struct NamedMapper;

impl NamedMapper {
    pub fn to_bo(model: NamedModel) -> NamedBO {
        NamedBO {
            id: model.id,
            container: model.container,
            context: model.context,
            name: model.name,
            language: model.language,
            created: model.created,
            user_details: model.user_details,
            short_desc: model.short_desc,
            long_desc: model.long_desc,
            container_type: match model.container_type {
                postings_db::models::named::ContainerType::Ledger => postings_api::domain::named::ContainerType::Ledger,
                postings_db::models::named::ContainerType::ChartOfAccount => postings_api::domain::named::ContainerType::ChartOfAccount,
                postings_db::models::named::ContainerType::LedgerAccount => postings_api::domain::named::ContainerType::LedgerAccount,
            }
        }
    }

    pub fn to_model(bo: NamedBO) -> NamedModel {
        NamedModel {
            id: bo.id,
            container: bo.container,
            context: bo.context,
            name: bo.name,
            language: bo.language,
            created: bo.created,
            user_details: bo.user_details,
            short_desc: bo.short_desc,
            long_desc: bo.long_desc,
            container_type: match bo.container_type {
                postings_api::domain::named::ContainerType::Ledger => postings_db::models::named::ContainerType::Ledger,
                postings_api::domain::named::ContainerType::ChartOfAccount => postings_db::models::named::ContainerType::ChartOfAccount,
                postings_api::domain::named::ContainerType::LedgerAccount => postings_db::models::named::ContainerType::LedgerAccount,
            }
        }
    }
}
