#![warn(clippy::unwrap_used)]

pub mod db;

use rocket::serde::{json::Json, Serialize};
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};

use crate::db::Db;

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    products: Vec<Product>,
    count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_min_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_max_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_avg_price: Option<f32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_price: Option<f32>,
}

#[get("/products?<search>&<category>&<certificate>&<origin>&<sort_by>")]
async fn list_products(
    mut db_con: Connection<Db>,
    search: Option<&str>,
    category: Option<&str>,
    certificate: Option<&str>,
    origin: Option<&str>,
    sort_by: Option<&str>,
) -> Json<ProductList> {
    let products =
        db::get_products_from_category(&mut db_con, search, category, certificate, origin, sort_by)
            .await;
    let (global_min_price, global_max_price, global_avg_price): (
        Option<f32>,
        Option<f32>,
        Option<f32>,
    ) = {
        if products.is_empty() {
            (None, None, None)
        } else {
            let mut min: Option<f32> = None;
            let mut max: Option<f32> = None;
            let mut sum: f64 = 0.0;
            products.iter().for_each(|p| {
                (p.price > max.unwrap_or(f32::MIN)).then(|| max = Some(p.price));
                (p.price < min.unwrap_or(f32::MAX)).then(|| min = Some(p.price));
                sum += p.price as f64;
            });
            (
                min,
                max,
                if sum == 0.0 {
                    None
                } else {
                    Some((sum / products.len() as f64) as f32)
                },
            )
        }
    };
    Json(ProductList {
        count: products.len() as u32,
        products,
        global_min_price,
        global_avg_price,
        global_max_price,
    })
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to Eatyoucate"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello, list_products])
        .attach(db::Db::init())
        .attach(rocket::fairing::AdHoc::on_response("CORS", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Access-Control-Allow-Origin", "*");
            })
        }))
        .launch()
        .await
        .expect("Failed to start Rocket");
}
