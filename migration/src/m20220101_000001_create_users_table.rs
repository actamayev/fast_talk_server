use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Credentials::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Credentials::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Credentials::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Credentials::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Credentials::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Credentials {
    Table,
    UserId,
    Username,
    Password,
    IsActive,
    CreatedAt,
}
