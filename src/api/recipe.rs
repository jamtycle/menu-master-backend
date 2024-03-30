use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{RecipeRequest, RecipeResponse},
};

use super::response::APIResponse;

pub fn recipe_routes() -> Vec<rocket::Route> {
    routes![
        get_all_recipes,
        get_recipe,
        create_recipe,
        update_recipe,
        delete_recipe
    ]
}

#[get("/")]
async fn get_all_recipes(db: &State<MongoDB>) -> Json<APIResponse<Vec<RecipeResponse>>> {
    Json(APIResponse::new_success_nm(db.get_all_recipes()))
}

#[get("/<iid>")]
async fn get_recipe(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<RecipeResponse>> {
    let id = ObjectId::parse_str(iid).unwrap();
    Json(APIResponse::new_success_nm(db.get_recipe(&id)))
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_recipe(
    info: Json<RecipeRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<String>> {
    let id = db.create_recipe(&info.0).unwrap();
    Json(APIResponse::new_success_nm(id.to_hex()))
}

#[put("/<iid>", format = "application/json", data = "<info>")]
async fn update_recipe(
    iid: &str,
    info: Json<RecipeRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    let id = ObjectId::parse_str(iid).unwrap();
    Json(APIResponse::new_success_nm(db.update_recipe(&id, &info.0)))
}

#[delete("/<iid>")]
async fn delete_recipe(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    let id = ObjectId::parse_str(iid).unwrap();
    Json(APIResponse::new_success_nm(db.delete_recipe(&id)))
}
