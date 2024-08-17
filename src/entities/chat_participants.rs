use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "chat_participants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub chat_participant_id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub joined_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Chats,
    Credentials,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Chats => Entity::belongs_to(super::chats::Entity)
                .from(Column::ChatId)
                .to(super::chats::Column::ChatId)
                .into(),
            Self::Credentials => Entity::belongs_to(super::credentials::Entity)
                .from(Column::UserId)
                .to(super::credentials::Column::UserId)
                .into(),
        }
    }
}

impl Related<super::chats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chats.def()
    }
}

impl Related<super::credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Credentials.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// TODO: Add a Unique index to the entity (between chatId and userId)
