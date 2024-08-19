use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub message_id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub text: String,
    pub sent_at: DateTimeWithTimeZone,
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
                .from(Column::SenderId)
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
