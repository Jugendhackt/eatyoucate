pub mod db;

use rocket::serde::{json::Json, Serialize};
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};

use crate::db::Db;

const PRODUCTS: [&str; 3] = ["banana", "apple", "rice"];

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    products: Vec<String>,
}

pub struct Product {
    name: &'static str,
    categories: Vec<&'static str>,
    certificate: Vec<&'static str>,
}

#[get("/products?<search>")]
async fn list_products(mut db_con: Connection<Db>, search: Option<&str>) -> Json<ProductList> {
    let mut out: Vec<String> = vec![];
    let products = db::get_categories(&mut db_con).await;
    match search {
        Some(t) => {
            for product in products {
                if product.contains(t) {
                    out.push(product)
                }
            }
        }
        None => {
            out = products;
        }
    }
    Json(ProductList { products: out })
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
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
