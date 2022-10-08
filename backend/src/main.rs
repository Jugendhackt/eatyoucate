use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, routes};
const PRODUCTS: [&str; 3] = ["banana", "apple", "rice"];

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    data: Vec<&'static str>,
}

#[get("/products?<term>")]
fn list_products(term: Option<&str>) -> Json<ProductList> {
    let mut out: Vec<&str> = vec![];
    match term {
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
    Json(ProductList { data: out })
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, list_products])
}
