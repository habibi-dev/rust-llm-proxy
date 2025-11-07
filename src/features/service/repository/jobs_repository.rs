use crate::features::service::model::jobs;
use crate::features::service::model::jobs::Model;
use crate::features::service::validation::job_form::JobForm;
use crate::utility::state::app_state;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

pub struct JobRepository;

impl JobRepository {
    pub async fn create(data: JobForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let job = jobs::ActiveModel {
            hash: Set(data.hash),
            model: Set(data.model),
            input: Set(data.input),
            retry: Set(0),
            output: Set(data.output),
            user_id: Set(data.user_id),
            api_key_id: Set(data.api_key_id),
            status: Set(data.status),
            finished_at: Set(data.finished_at),
            ..Default::default()
        };

        match job.insert(&state._db).await {
            Ok(model) => Ok(Some(model)),
            Err(err) => Err(err),
        }
    }

    pub async fn update(job_id: i64, data: JobForm) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let Some(_job) = Self::find_by_id(job_id).await? else {
            return Ok(None);
        };

        let am = jobs::ActiveModel {
            id: Set(job_id),
            hash: Set(data.hash),
            model: Set(data.model),
            input: Set(data.input),
            retry: Set(data.retry),
            output: Set(data.output),
            user_id: Set(data.user_id),
            api_key_id: Set(data.api_key_id),
            status: Set(data.status),
            finished_at: Set(data.finished_at),
            ..Default::default()
        };

        let updated = am.update(&state._db).await?;
        Ok(Some(updated))
    }

    pub async fn update_output_status(
        job_id: i64,
        output: Option<sea_orm::prelude::Json>,
        status: jobs::JobStatus,
        finished_at: Option<sea_orm::prelude::DateTime>,
    ) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let Some(_job) = Self::find_by_id(job_id).await? else {
            return Ok(None);
        };

        let am = jobs::ActiveModel {
            id: Set(job_id),
            output: Set(output),
            status: Set(status),
            finished_at: Set(finished_at),
            ..Default::default()
        };

        let updated = am.update(&state._db).await?;
        Ok(Some(updated))
    }

    pub async fn find_by_id(job_id: i64) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let job = jobs::Entity::find_by_id(job_id).one(&state._db).await?;

        Ok(job)
    }

    pub async fn find_by_hash(hash: &str) -> Result<Option<Model>, DbErr> {
        let state = app_state();

        let job = jobs::Entity::find()
            .filter(jobs::Column::Hash.eq(hash))
            .one(&state._db)
            .await?;

        Ok(job)
    }

    pub async fn delete(job_id: i64) -> Result<bool, DbErr> {
        let state = app_state();

        let job = Self::find_by_id(job_id).await?;

        if let Some(job) = job {
            let am: jobs::ActiveModel = job.into();
            am.delete(&state._db).await?;
        }

        Ok(true)
    }
}
