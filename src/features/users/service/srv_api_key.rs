use crate::core::state::APP_STATE;
use crate::features::users::model::user;
use crate::features::users::model::user::Model;
use crate::features::users::repository::repo_api_key::RepositoryApiKey;
use sea_orm::ModelTrait;

pub struct ServiceApiKey;
impl ServiceApiKey {
    pub async fn is_admin(raw_key: &str) -> bool {
        let Some(api_key) = RepositoryApiKey::get_by_key(raw_key).await else {
            return false;
        };

        let _state = APP_STATE.get().expect("App state not initialized");

        if let Ok(Some(user)) = api_key.find_related(user::Entity).one(&_state._db).await {
            return user.is_admin;
        }

        false
    }

    pub async fn auth(raw_key: &str) -> Option<Model> {
        let api_key = RepositoryApiKey::get_by_key(raw_key).await?;

        let _state = APP_STATE.get().expect("App state not initialized");

        if let Ok(Some(user)) = api_key.find_related(user::Entity).one(&_state._db).await {
            return Some(user);
        }

        None
    }
}
