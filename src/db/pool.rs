use sea_orm::{DatabaseConnection, DbErr};

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = String::from("postgres://postgres:postgres@localhost:5432/database");
    sea_orm::Database::connect(&db_url).await
}
