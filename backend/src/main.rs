use rocket::serde::{json::Json, Serialize};
use rocket::{get, routes};
use rocket_db_pools::Database;
const PRODUCTS: [&str; 3] = ["banana", "apple", "rice"];

#[derive(Database)]
#[database("data")]
pub struct Db(sqlx::SqlitePool);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    products: Vec<&'static str>,
}

pub struct Product {
    name: &'static str,
    categories: Vec<&'static str>,
    certificate: Vec<&'static str>,
}

#[get("/products?<search>")]
fn list_products(search: Option<&str>) -> Json<ProductList> {
    let mut out: Vec<&str> = vec![];
    match search {
        Some(t) => {
            for product in PRODUCTS {
                if product.contains(t) {
                    out.push(product)
                }
            }
        }
        None => {
            out = PRODUCTS.to_vec();
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
        .attach(Db::init())
        .launch()
        .await
        .unwrap();
}
