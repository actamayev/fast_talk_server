use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the `credentials` table
        manager.create_table(
            Table::create()
                .table(Credentials::Table)
                .if_not_exists()
                .col(ColumnDef::new(Credentials::UserId).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(Credentials::Username).string().not_null())
                .col(ColumnDef::new(Credentials::Password).string().not_null())
                .col(ColumnDef::new(Credentials::Email).string().not_null())
                .col(ColumnDef::new(Credentials::IsActive).boolean().not_null())
                .col(ColumnDef::new(Credentials::CreatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;

        // Create the `chats` table
        manager.create_table(
            Table::create()
                .table(Chats::Table)
                .if_not_exists()
                .col(ColumnDef::new(Chats::ChatId).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(Chats::LastMessage).string().null())
                .col(ColumnDef::new(Chats::LastMessageSenderId).integer().null())
                .col(ColumnDef::new(Chats::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Chats::UpdatedAt).timestamp_with_time_zone().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_chats_last_message_sender_id")
                        .from(Chats::Table, Chats::LastMessageSenderId)
                        .to(Credentials::Table, Credentials::UserId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await?;

        // Create the `chat_participants` table
        manager.create_table(
            Table::create()
                .table(ChatParticipants::Table)
                .if_not_exists()
                .col(ColumnDef::new(ChatParticipants::ChatParticipantId).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(ChatParticipants::ChatId).integer().not_null())
                .col(ColumnDef::new(ChatParticipants::UserId).integer().not_null())
                .col(ColumnDef::new(ChatParticipants::JoinedAt).timestamp_with_time_zone().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_chat_participants_chat_id")
                        .from(ChatParticipants::Table, ChatParticipants::ChatId)
                        .to(Chats::Table, Chats::ChatId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_chat_participants_user_id")
                        .from(ChatParticipants::Table, ChatParticipants::UserId)
                        .to(Credentials::Table, Credentials::UserId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await?;

        // Create the `login_history` table
        manager.create_table(
            Table::create()
                .table(LoginHistory::Table)
                .if_not_exists()
                .col(ColumnDef::new(LoginHistory::LoginHistoryId).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(LoginHistory::UserId).integer().not_null())
                .col(ColumnDef::new(LoginHistory::LoginTime).timestamp_with_time_zone().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_login_history_user_id")
                        .from(LoginHistory::Table, LoginHistory::UserId)
                        .to(Credentials::Table, Credentials::UserId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await?;

        // Create the `messages` table
        manager.create_table(
            Table::create()
                .table(Messages::Table)
                .if_not_exists()
                .col(ColumnDef::new(Messages::MessageId).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(Messages::ChatId).integer().not_null())
                .col(ColumnDef::new(Messages::SenderId).integer().not_null())
                .col(ColumnDef::new(Messages::Text).string().not_null())
                .col(ColumnDef::new(Messages::SentAt).timestamp_with_time_zone().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_messages_chat_id")
                        .from(Messages::Table, Messages::ChatId)
                        .to(Chats::Table, Chats::ChatId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_messages_sender_id")
                        .from(Messages::Table, Messages::SenderId)
                        .to(Credentials::Table, Credentials::UserId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation to avoid foreign key conflicts
        manager.drop_table(Table::drop().table(Messages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(LoginHistory::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ChatParticipants::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Chats::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Credentials::Table).to_owned()).await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Chats {
    Table,
    ChatId,
    LastMessage,
    LastMessageSenderId,
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
enum Credentials {
    Table,
    UserId,
    Username,
    Password,
    Email,
    IsActive,
    CreatedAt,
}

#[derive(DeriveIden)]
enum LoginHistory {
    Table,
    LoginHistoryId,
    UserId,
    LoginTime,
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
