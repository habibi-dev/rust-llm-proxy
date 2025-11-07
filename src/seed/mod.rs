pub mod insert_admin;

pub async fn run_all(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    insert_admin::run(db).await
}
