pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20240815_214045_login_history;
mod m20240815_221219_add_email_to_credentials;
mod m20240816_200240_create_chat_tables;
mod m20240817_175500_add_unique_constraint_on_chat_participants;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20240815_214045_login_history::Migration),
            Box::new(m20240815_221219_add_email_to_credentials::Migration),
            Box::new(m20240816_200240_create_chat_tables::Migration),
            Box::new(m20240817_175500_add_unique_constraint_on_chat_participants::Migration),
        ]
    }
}
