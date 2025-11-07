use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Service::Id)
                            .big_unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Service::Title).string().not_null())
                    .col(ColumnDef::new(Service::Provider).string().not_null())
                    .col(ColumnDef::new(Service::Key).string().null())
                    .col(ColumnDef::new(Service::Status).boolean().default(true))
                    .col(
                        ColumnDef::new(Service::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Service::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Service {
    Table,
    Id,
    Title,
    Provider,
    Key,
    Status,
    CreatedAt,
}
