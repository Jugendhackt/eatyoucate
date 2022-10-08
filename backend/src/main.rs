pub mod db;

use rocket::serde::{json::Json, Serialize};
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};

use crate::db::Db;

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    products: Vec<Product>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    category_name: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    unit: String,
    price: f32,
    amount: f32,
    origin: Option<String>,
}

#[get("/products?<search>&<category>&<certificate>&<origin>")]
async fn list_products(
    mut db_con: Connection<Db>,
    search: Option<&str>,
    category: Option<&str>,
    certificate: Option<&str>,
    origin: Option<&str>,
) -> Json<ProductList> {
    Json(ProductList {
        products: db::get_products_from_category(
            &mut db_con,
            search,
            category,
            certificate,
            origin,
        )
        .await,
    })
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to Eatyoucate"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![hello, list_products])
        .attach(db::Db::init())
        .launch()
        .await
        .unwrap();
}
