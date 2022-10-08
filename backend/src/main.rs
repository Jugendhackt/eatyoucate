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

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    name: &'static str,
    categories: Vec<&'static str>,
    certificates: Vec<&'static str>,
}

#[get("/products?<search>&<category>&<certificate>")]
fn list_products(
    search: Option<&str>,
    category: Option<&str>,
    certificate: Option<&str>,
) -> Json<ProductList> {
    let mut out: Vec<Product> = vec![];

    if search.is_none() && category.is_none() && certificate.is_none() {
        out = dummy_products().to_vec();
    } else {
        let mut valid: bool;
        for product in dummy_products() {
            valid = true;

            match search {
                Some(s) => {
                    if !product.name.contains(s) {
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
