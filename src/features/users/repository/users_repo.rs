use crate::features::users::model::prelude::User;
use crate::features::users::model::user;
use crate::features::users::model::user::Model;
use crate::features::users::validation::user_form::UserForm;
use crate::utility::state::app_state;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, Set};

pub struct UsersRepository;
impl UsersRepository {
    pub async fn all() -> Option<Vec<Model>> {
        let state = app_state();

        User::find()
            .order_by_desc(user::Column::Id)
            .all(&state._db)
            .await
            .ok()
    }

    pub async fn create(data: UserForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let user = user::ActiveModel {
            name: Set(data.name),
            status: Set(data.status.unwrap_or(true)),
            is_admin: Set(data.is_admin.unwrap_or(false)),
            // created_at: Set(chrono::Utc::now().into()),
            // updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        match user.insert(&state._db).await {
            Ok(model) => Ok(Some(model)),
            Err(err) => Err(err),
        }
    }

    pub async fn update(user_id: i64, data: UserForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let Some(_user) = Self::find_by_id(&state._db, user_id).await? else {
            return Ok(None);
        };

        let am = user::ActiveModel {
            id: Set(user_id),
            name: Set(data.name),
            status: Set(data.status.unwrap_or(true)),
            is_admin: Set(data.is_admin.unwrap_or(false)),
            ..Default::default()
        };

        let updated = am.update(&state._db).await?;
        Ok(Some(updated))
    }

    pub async fn delete(user_id: i64) -> Result<bool, String> {
        let state = app_state();

        let Some(_user) = Self::find_by_id(&state._db, user_id)
            .await
            .map_err(|e| e.to_string())?
        else {
            return Err("User not found".to_string());
        };

        match user::Entity::delete_by_id(user_id).exec(&state._db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<Model>, DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }
}
