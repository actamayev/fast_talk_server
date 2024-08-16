use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "login_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub login_history_id: i32,
    pub user_id: i32,
    pub login_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Credentials,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Credentials => Entity::belongs_to(super::credentials::Entity)
                .from(Column::UserId)
                .to(super::credentials::Column::UserId)
                .into(),
        }
    }
}

impl Related<super::credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Credentials.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
