use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LoginHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoginHistory::LoginHistoryId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoginHistory::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(LoginHistory::LoginTime)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-login_history-user_id")
                            .from(LoginHistory::Table, LoginHistory::UserId)
                            .to(Credentials::Table, Credentials::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoginHistory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum LoginHistory {
    Table,
    LoginHistoryId,
    UserId,
    LoginTime,
}

#[derive(Iden)]
enum Credentials {
    Table,
    UserId,
}
