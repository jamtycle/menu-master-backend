use rocket::{serde::json::Json, State};

use crate::{db::mongodb::MongoDB, model::restaurants::Restaurant};

use super::response::APIResponse;

pub fn restaurant_routes() -> Vec<rocket::Route> {
    routes![get_all_restaurant]
}

#[get("/")]
async fn get_all_restaurant(db: &State<MongoDB>) -> Json<APIResponse<Option<Vec<Restaurant>>>> {
    Json(APIResponse {
        code: 200,
        data: db.get_all_restaurants(),
        message: "".to_string(),
    })
}
