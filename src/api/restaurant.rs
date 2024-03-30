use rocket::{serde::json::Json, State};

use crate::{db::mongodb::MongoDB, model::restaurants::RestaurantResponse};

use super::response::APIResponse;

pub fn restaurant_routes() -> Vec<rocket::Route> {
    routes![get_all_restaurant]
}

#[get("/")]
async fn get_all_restaurant(db: &State<MongoDB>) -> Json<APIResponse<Vec<RestaurantResponse>>> {
    Json(APIResponse::new_success_nm(db.get_all_restaurants()))
}
