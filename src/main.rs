pub mod config;
pub mod db;
pub mod model;
pub mod api;

#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use api::{user::{get_users, login_user, register_user}, response::APIResponse};
use config::{cors::CORS, restful::RESTFul};
use db::mongodb::MongoDB;
use rocket::{
    http::Status,
    tokio::time::{sleep, Duration},
    Request, serde::json::Json,
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
        message: "Server error handler.".to_string()
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
        .mount("/user", routes![get_users, login_user, register_user])
}

fn generate_db() -> MongoDB {
    MongoDB::init()
}
