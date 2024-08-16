use sea_orm::Database;
use std::env;
use dotenvy::dotenv;

pub async fn establish_connection() -> sea_orm::DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
