use crate::m20251016_092534_create_users_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ApiKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKey::Id)
                            .big_unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKey::UserId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(ApiKey::KeyHash)
                            .string_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(ApiKey::Status)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ApiKey::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ApiKey::Table, ApiKey::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKey::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ApiKey {
    Table,
    Id,
    UserId,
    KeyHash,
    Status,
    CreatedAt,
}
