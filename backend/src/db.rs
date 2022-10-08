use rocket_db_pools::Database;
use sqlx::query;

#[derive(Database)]
#[database("data")]
pub struct Db(sqlx::SqlitePool);

macro_rules! fetch_all {
    ($query: expr, $db: expr, $field: ident) => {
        $query
            .fetch_all($db)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.$field)
            .collect()
    };
}

pub async fn get_products_from_category(
    db: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    category: &str,
) -> Vec<String> {
    fetch_all!(
        query!(
            "select *
from Kategorien k, Produkte p, Produktpreise pp
where k.Name = ifnull(?, k.Name)
and p.Kat_ID = k.ID
and pp.Prd_ID = p.ID
and p.name like ?
and pp.Herkunft = ifnull(?, pp.Herkunft)
and pp.Zertifikat = ifnull(?, pp.Zertifikat)",
            category,
            category,
            category
        ),
        db,
        Name
    )
}
