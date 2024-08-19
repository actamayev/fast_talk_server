use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub chat_id: i32,
    pub last_message: Option<String>, // Made optional to allow null values
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ChatParticipants,
    Messages,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ChatParticipants => Entity::has_many(super::chat_participants::Entity).into(),
            Self::Messages => Entity::has_many(super::messages::Entity).into(),
        }
    }
}

impl Related<super::chat_participants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatParticipants.def()
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
