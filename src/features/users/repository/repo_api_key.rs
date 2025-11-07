use crate::features::users::model::api_key;
use crate::features::users::model::api_key::{Column, Entity, Model};
use crate::features::users::model::prelude::ApiKey;
use crate::features::users::utility::hash_key::hash_key;
use crate::features::users::validation::api_key_form::ApiKeyForm;
use crate::utility::state::app_state;
use sea_orm::sea_query::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

pub struct RepositoryApiKey;

impl RepositoryApiKey {
    pub async fn all() -> Option<Vec<Model>> {
        let state = app_state();

        ApiKey::find()
            .order_by_desc(Column::Id)
            .all(&state._db)
            .await
            .ok()
    }

    pub async fn create(data: ApiKeyForm) -> Result<(Option<Model>, String), DbErr> {
        let state = app_state();
        let raw_key = data.key.clone().expect("key is required");

        let api_key = api_key::ActiveModel {
            key_hash: Set(data.key_hash.expect("key_hash is required")),
            user_id: Set(data.user_id.expect("user_id is required")),
            status: Set(data.status.unwrap_or(true)),
            ..Default::default()
        };

        match api_key.insert(&state._db).await {
            Ok(model) => Ok((Some(model), raw_key)),
            Err(err) => Err(err),
        }
    }

    pub async fn update_status(raw_key: &str, status: bool) -> Result<bool, DbErr> {
        let state = app_state();

        let result = Entity::update_many()
            .col_expr(Column::Status, Expr::value(status))
            .filter(Column::KeyHash.eq(raw_key))
            .exec(&state._db)
            .await?;

        Ok(result.rows_affected > 0)
    }

    pub async fn delete(raw_key: &str) -> Result<bool, String> {
        let state = app_state();

        let Some(apikey) = Self::get_by_key(raw_key).await else {
            return Err("Api key not found".to_string());
        };

        match Entity::delete_by_id(apikey.id).exec(&state._db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_by_key(raw_key: &str) -> Option<Model> {
        let key = hash_key(raw_key);
        let state = app_state();

        Entity::find()
            .filter(Column::KeyHash.eq(key))
            .one(&state._db)
            .await
            .ok()
            .flatten()
    }

    pub async fn key_exist(raw_key: &str) -> bool {
        Self::get_by_key(raw_key).await.is_some()
    }
}
