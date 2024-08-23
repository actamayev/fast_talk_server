use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "credentials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    LoginHistory,
    Chats,
    ChatParticipants,
    Messages
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::LoginHistory => Entity::has_many(super::login_history::Entity).into(),
            Self::Chats => Entity::has_many(super::chats::Entity).into(),
            Self::ChatParticipants => Entity::has_many(super::chat_participants::Entity).into(),
            Self::Messages => Entity::has_many(super::messages::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
