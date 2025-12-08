use crate::features::service::constants::{DEFAULT_DEEPSEEK_MODEL, DEFAULT_GEMINI_MODEL};
use crate::features::service::model::prelude::Service;
use crate::features::service::model::service;
use crate::features::service::model::service::{Column, Model};
use crate::features::service::validation::service_form::ServiceForm;
use crate::utility::state::app_state;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QueryOrder, Set};

pub struct ServiceRepository;
impl ServiceRepository {
    pub async fn list() -> Option<Vec<Model>> {
        let state = app_state();

        Service::find()
            .order_by_desc(Column::Id)
            .all(&state._db)
            .await
            .ok()
    }

    pub async fn create(data: ServiceForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();
        let provider = data.provider;
        let model = Self::resolve_model(&provider, data.model, None);
        let service = service::ActiveModel {
            title: Set(data.title),
            provider: Set(provider.clone()),
            key: Set(data.key.expect("key is required")),
            model: Set(model),
            status: Set(data.status.unwrap_or(false)),
            ..Default::default()
        };

        match service.insert(&state._db).await {
            Ok(model) => Ok(Some(model)),
            Err(err) => Err(err),
        }
    }

    pub async fn update(service_id: i64, data: ServiceForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let Some(existing_service) = service::Entity::find_by_id(service_id)
            .one(&state._db)
            .await?
        else {
            return Ok(None);
        };

        let provider = data.provider;
        let model =
            Self::resolve_model(&provider, data.model, Some(existing_service.model.clone()));

        let am = service::ActiveModel {
            id: Set(service_id),
            title: Set(data.title),
            provider: Set(provider),
            key: Set(data.key.expect("key is required")),
            model: Set(model),
            status: Set(data.status.expect("status is required")),
            ..Default::default()
        };

        let updated = am.update(&state._db).await?;
        Ok(Some(updated))
    }

    pub async fn delete(service_id: i64) -> Result<bool, String> {
        let state = app_state();

        let Some(_service) = service::Entity::find_by_id(service_id)
            .one(&state._db)
            .await
            .map_err(|e| e.to_string())?
        else {
            return Err("Service not found".to_string());
        };

        match service::Entity::delete_by_id(service_id)
            .exec(&state._db)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn find_by_id(service_id: i64) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let service = service::Entity::find_by_id(service_id)
            .one(&state._db)
            .await?;

        Ok(service)
    }

    fn resolve_model(provider: &str, model: Option<String>, existing: Option<String>) -> String {
        if let Some(current) = model.filter(|value| !value.trim().is_empty()) {
            return current;
        }

        if let Some(existing_model) = existing.filter(|value| !value.trim().is_empty()) {
            return existing_model;
        }

        match provider {
            "deepseek" => DEFAULT_DEEPSEEK_MODEL.to_string(),
            _ => DEFAULT_GEMINI_MODEL.to_string(),
        }
    }
}
