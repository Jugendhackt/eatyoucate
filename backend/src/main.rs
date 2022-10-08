pub mod db;

use rocket::serde::{json::Json, Serialize};
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};

use crate::db::Db;

const PRODUCTS: [&str; 3] = ["banana", "apple", "rice"];

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    products: Vec<Product>,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    category_name: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    price: f32,
    amount: u16,
    origin: String,
}

#[get("/products?<search>&<category>&<certificate>")]
async fn list_products(
    mut db_con: Connection<Db>,
    search: Option<&str>,
    category: Option<&str>,
    certificate: Option<&str>,
) -> Json<ProductList> {
    let mut out: Vec<String> = vec![];
    let categories = db::get_categories(&mut db_con).await;
    if search.is_none() && category.is_none() && certificate.is_none() {
        out = categories;
    } else {
        let mut valid: bool;
        for product in categories {
            valid = true;

            match search {
                Some(s) => {
                    if !product.contains(s) {
                        valid = false;
                    }
                }
                _ => {}
            }
            match category {
                Some(c) => {
                    if !product.categories.contains(&c) {
                        valid = false;
                    }
                }
                _ => {}
            }
            match certificate {
                Some(c) => {
                    if !product.certificates.contains(&c) {
                        valid = false;
                    }
                }
                _ => {}
            }

            if valid {
                out.push(product)
            }
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
