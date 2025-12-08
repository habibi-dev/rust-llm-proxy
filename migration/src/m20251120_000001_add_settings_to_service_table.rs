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
                        ColumnDef::new(Service::Settings)
                            .json()
                            .not_null()
                            .default("{}"),
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
                    .drop_column(Service::Settings)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Service {
    Table,
    Settings,
}
