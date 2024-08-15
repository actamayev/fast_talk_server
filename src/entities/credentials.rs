use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "credentials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    LoginHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::LoginHistory => Entity::has_many(super::login_history::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
