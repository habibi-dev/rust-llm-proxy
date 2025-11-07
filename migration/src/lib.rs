mod m20251016_092534_create_users_table;
mod m20251016_173133_create_api_keys_table;
mod m20251026_060001_service_table;
mod m20251102_194252_create_jobs_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251016_092534_create_users_table::Migration),
            Box::new(m20251016_173133_create_api_keys_table::Migration),
            Box::new(m20251026_060001_service_table::Migration),
            Box::new(m20251102_194252_create_jobs_table::Migration),
        ]
    }
}
