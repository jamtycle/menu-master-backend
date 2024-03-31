pub mod api;
pub mod config;
pub mod db;
pub mod model;

#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use api::{
    ingredients::ingredient_routes, inventory::inventory_routes, menu::menu_routes,
    prep_list::prep_list_routes, product_order::product_order_routes, recipe::recipe_routes,
    response::APIResponse, restaurant::restaurant_routes, user::user_routes,
};
use config::restful::RESTFul;
use db::mongodb::MongoDB;
use rocket::{
    http::{Method, Status},
    serde::json::Json,
    tokio::time::{sleep, Duration},
    Request,
};
use rocket_cors::CorsOptions;

#[get("/")]
async fn index() -> Json<HashMap<String, String>> {
    let mut info: HashMap<String, String> = HashMap::new();
    info.insert("message".to_string(), "Hello World!".to_string());
    Json(info)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[catch(default)]
async fn error_handler(_status: Status, _request: &Request<'_>) -> Json<APIResponse<String>> {
    let response = APIResponse {
        code: _status.code,
        data: Some(_status.reason().unwrap_or("Server Error").to_string()),
        message: "Server error handler.".to_string(),
    };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    let cors = cors_options()
        .to_cors()
        .expect("CORS configuration failed.");

    rocket::build()
        .attach(RESTFul)
        .attach(cors)
        .manage(generate_db())
        .register("/", catchers![error_handler])
        .mount("/", routes![index, delay])
        .mount("/ingredient", ingredient_routes())
        .mount("/inventory", inventory_routes())
        .mount("/menu", menu_routes())
        .mount("/preplist", prep_list_routes())
        .mount("/productorder", product_order_routes())
        .mount("/recipe", recipe_routes())
        .mount("/restaurant", restaurant_routes())
        .mount("/user", user_routes())
}

fn generate_db() -> MongoDB {
    MongoDB::init()
}

fn cors_options() -> CorsOptions {
    CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::All,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allow_credentials: true,
        allowed_headers: rocket_cors::AllOrSome::All,
        ..Default::default()
    }
}
