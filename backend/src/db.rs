use rocket_db_pools::Database;
use sqlx::query;

use crate::Product;

#[derive(Database)]
#[database("data")]
pub struct Db(sqlx::SqlitePool);

macro_rules! fetch_all {
    ($query: expr, $db: expr) => {
        $query.fetch_all($db).await.unwrap()
    };
}

pub async fn get_products_from_category(
    db: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    search: Option<&str>,
    category: Option<&str>,
    certificate: Option<&str>,
    origin: Option<&str>,
) -> Vec<Product> {
    fetch_all!(
        query!(
            "select KAT_NAME,
PRD_NAME,
PPR_PREIS,
PPR_MENGE,
PPR_EINHEIT,
PPR_HERKUNFT,
PPR_ZERTIFIKAT
from Kategorien, Produkte, Produktpreise
where KAT_NAME = ifnull(?, KAT_NAME)
and PRD_KAT_ID = KAT_ID
and PPR_PRD_ID = PRD_ID
and PRD_NAME like ifnull(?, PRD_NAME)
and PPR_HERKUNFT = ifnull(?, PPR_HERKUNFT)
and (PPR_ZERTIFIKAT = ifnull(?, PPR_ZERTIFIKAT) or PPR_ZERTIFIKAT is NULL)",
            category,
            search,
            origin,
            certificate
        ),
        db
    )
    .into_iter()
    .map(|x| Product {
        amount: x.PPR_MENGE,
        category_name: x.KAT_NAME,
        origin: x.PPR_HERKUNFT,
        price: x.PPR_PREIS,
        certificate: x.PPR_ZERTIFIKAT,
        name: x.PRD_NAME,
        unit: x.PPR_EINHEIT,
    })
    .collect()
}
