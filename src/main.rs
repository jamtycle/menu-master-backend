pub mod api;
pub mod config;
pub mod db;
pub mod model;

#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use api::{
    ingredients::ingredient_routes, inventory::inventory_routes, prep_list::prep_list_routes,
    recipe::recipe_routes, response::APIResponse, user::user_routes,
};
use config::{cors::CORS, restful::RESTFul};
use db::mongodb::MongoDB;
use rocket::{
    http::Status,
    serde::json::Json,
    tokio::time::{sleep, Duration},
    Request,
};

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
        data: _status.reason().unwrap_or("Server Error").to_string(),
        message: "Server error handler.".to_string(),
    };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RESTFul)
        .attach(CORS)
        .manage(generate_db())
        .register("/", catchers![error_handler])
        .mount("/", routes![index, delay])
        .mount("/user", user_routes())
        .mount("/ingedient", ingredient_routes())
        .mount("/inventory", inventory_routes())
        .mount("/preplist", prep_list_routes())
        .mount("/recipe", recipe_routes())
}

fn generate_db() -> MongoDB {
    MongoDB::init()
}
