use rocket_db_pools::Database;
use sqlx::prelude::*;

#[derive(Database)]
#[database("data")]
pub struct Db(sqlx::SqlitePool);

pub async fn get_categories(db: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>) -> Vec<String> {
    sqlx::query!("SELECT ID, Name FROM Produktkategorien")
        .fetch_all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.Name)
        .collect()
}
