use crate::features::users::model::{api_key, user};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub hash: String,
    pub model: String,
    pub input: Json,
    pub retry: i8,
    pub output: Option<Json>,
    pub user_id: i64,
    pub api_key_id: i64,
    pub status: JobStatus, // queued | running | done | failed
    pub created_at: DateTime,
    pub finished_at: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    #[sea_orm(string_value = "queued")]
    Queued,
    #[sea_orm(string_value = "running")]
    Running,
    #[sea_orm(string_value = "done")]
    Done,
    #[sea_orm(string_value = "failed")]
    Failed,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "api_key::Entity",
        from = "Column::ApiKeyId",
        to = "api_key::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ApiKey,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
