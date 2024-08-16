use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Chats table
        manager
            .create_table(
                Table::create()
                    .table(Chats::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Chats::ChatId).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Chats::LastMessage).string())
                    .col(ColumnDef::new(Chats::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Chats::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create ChatParticipants table
        manager
            .create_table(
                Table::create()
                    .table(ChatParticipants::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChatParticipants::ChatParticipantId).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(ChatParticipants::ChatId).integer().not_null())
                    .col(ColumnDef::new(ChatParticipants::UserId).integer().not_null())
                    .col(ColumnDef::new(ChatParticipants::JoinedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_participants-chat_id")
                            .from(ChatParticipants::Table, ChatParticipants::ChatId)
                            .to(Chats::Table, Chats::ChatId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_participants-user_id")
                            .from(ChatParticipants::Table, ChatParticipants::UserId)
                            .to(Credentials::Table, Credentials::UserId), // Assuming 'Credentials' is the table for users
                    )
                    .to_owned(),
            )
            .await?;

        // Create Messages table
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Messages::MessageId).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Messages::ChatId).integer().not_null())
                    .col(ColumnDef::new(Messages::SenderId).integer().not_null())
                    .col(ColumnDef::new(Messages::Text).string().not_null())
                    .col(ColumnDef::new(Messages::SentAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-chat_id")
                            .from(Messages::Table, Messages::ChatId)
                            .to(Chats::Table, Chats::ChatId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-sender_id")
                            .from(Messages::Table, Messages::SenderId)
                            .to(Credentials::Table, Credentials::UserId), // Assuming 'Credentials' is the table for users
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Messages table
        manager.drop_table(Table::drop().table(Messages::Table).to_owned()).await?;

        // Drop ChatParticipants table
        manager.drop_table(Table::drop().table(ChatParticipants::Table).to_owned()).await?;

        // Drop Chats table
        manager.drop_table(Table::drop().table(Chats::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Chats {
    Table,
    ChatId,
    LastMessage,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ChatParticipants {
    Table,
    ChatParticipantId,
    ChatId,
    UserId,
    JoinedAt,
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    MessageId,
    ChatId,
    SenderId,
    Text,
    SentAt,
}

#[derive(DeriveIden)]
enum Credentials {
    Table,
    UserId,
}
