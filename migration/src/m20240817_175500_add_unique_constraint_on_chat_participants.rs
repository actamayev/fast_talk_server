use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add a unique index on the chat_id and user_id columns
        manager
            .create_index(
                Index::create()
                    .name("idx-unique-user-chat")
                    .table(ChatParticipants::Table)
                    .col(ChatParticipants::UserId)
                    .col(ChatParticipants::ChatId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the unique index
        manager
            .drop_index(
                Index::drop()
                    .name("idx-unique-user-chat")
                    .table(ChatParticipants::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ChatParticipants {
    Table,
    ChatParticipantId,
    ChatId,
    UserId,
    JoinedAt,
}
