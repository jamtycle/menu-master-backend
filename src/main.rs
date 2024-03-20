pub mod api;
pub mod config;
pub mod db;
pub mod model;

#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use api::{
    ingredients::*,
    inventory::{
        create_inventory, delete_inventory, get_all_inventory, get_inventory, update_inventory,
    },
    prep_list::{
        create_prep_list, delete_prep_list, get_all_prep_list, get_prep_list, update_prep_list,
    },
    recipe::{create_recipe, delete_recipe, get_all_recipes, get_recipe, update_recipe},
    response::APIResponse,
    user::{get_users, login_user, register_user},
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
        .mount("/user", routes![get_users, login_user, register_user])
        .mount(
            "/ingedient",
            routes![
                get_all_ingredients,
                get_ingredint_by_id,
                create_ingredient,
                update_ingredient,
                delete_ingredient
            ],
        )
        .mount(
            "/inventory",
            routes![
                get_all_inventory,
                get_inventory,
                create_inventory,
                update_inventory,
                delete_inventory
            ],
        )
        .mount(
            "/preplist",
            routes![
                get_all_prep_list,
                get_prep_list,
                create_prep_list,
                update_prep_list,
                delete_prep_list
            ],
        )
        .mount(
            "/recipe",
            routes![
                get_all_recipes,
                get_recipe,
                create_recipe,
                update_recipe,
                delete_recipe
            ],
        )
}

fn generate_db() -> MongoDB {
    MongoDB::init()
}
