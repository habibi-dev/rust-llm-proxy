use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Service::Table)
                    .add_column(
                        ColumnDef::new(Service::Model)
                            .string()
                            .not_null()
                            .default("gemini-2.0-flash-lite"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Service::Table)
                    .drop_column(Service::Model)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Service {
    Table,
    Model,
}
