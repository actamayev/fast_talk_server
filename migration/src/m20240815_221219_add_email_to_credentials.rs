use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Credentials::Table)
                    .add_column(
                        ColumnDef::new(Credentials::Email)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Credentials::Table)
                    .drop_column(Credentials::Email)
                    .to_owned(),
            )
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
    Email, // Added the new email column identifier
}
