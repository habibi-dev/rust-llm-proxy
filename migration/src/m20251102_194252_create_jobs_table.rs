use crate::m20251016_092534_create_users_table::User;
use crate::m20251016_173133_create_api_keys_table::ApiKey;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Jobs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Jobs::Id)
                            .big_unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Jobs::Hash)
                            .string()
                            .string_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Jobs::Model).string().not_null())
                    .col(ColumnDef::new(Jobs::Input).text().not_null())
                    .col(ColumnDef::new(Jobs::Retry).integer().default(0))
                    .col(ColumnDef::new(Jobs::Output).text().json().null())
                    .col(
                        ColumnDef::new(Jobs::Status)
                            .string()
                            .not_null()
                            .default("queued"),
                    )
                    .col(ColumnDef::new(Jobs::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(Jobs::ApiKeyId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(Jobs::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Jobs::FinishedAt).date_time().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Jobs::Table, Jobs::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Jobs::Table, Jobs::ApiKeyId)
                            .to(ApiKey::Table, ApiKey::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Jobs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Jobs {
    Table,
    Id,
    Hash,
    Model,
    Input,
    Output,
    Retry,
    Status,
    UserId,
    ApiKeyId,
    CreatedAt,
    FinishedAt,
}
