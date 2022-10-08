use sqlx::prelude::*;

const DB_FILE: &str = "../../Datenbank/datenbank.db";

pub fn init_db() {
    Connection::connect(DB_FILE).await?
}

pub fn get_categories() -> Vec<&str> {}
