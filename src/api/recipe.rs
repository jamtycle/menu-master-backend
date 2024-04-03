use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{RecipeRequest, RecipeResponse},
};

use super::{
    response::{ok, ApiResult},
    utils::parse_object_id,
};

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
async fn get_all_recipes(db: &State<MongoDB>) -> ApiResult<Vec<RecipeResponse>> {
    ok(db.get_all_recipes().await?)
    // Json(APIResponse::new_success_nm(db.get_all_recipes()))
}

#[get("/<iid>")]
async fn get_recipe(iid: &str, db: &State<MongoDB>) -> ApiResult<RecipeResponse> {
    let id = parse_object_id(iid)?;
    // Json(APIResponse::new_success_nm(db.get_recipe(&id)))
    ok(db.get_recipe(&id).await?)
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_recipe(info: Json<RecipeRequest>, db: &State<MongoDB>) -> ApiResult<String> {
    let data = db.create_recipe(&info.0).await?;

    ok(data.to_hex())
}

#[put("/<iid>", format = "application/json", data = "<info>")]
async fn update_recipe(
    iid: &str,
    info: Json<RecipeRequest>,
    db: &State<MongoDB>,
) -> ApiResult<bool> {
    let id = parse_object_id(iid)?;
    let data = db.update_recipe(&id, &info.0).await?;

    ok(data)
}

#[delete("/<iid>")]
async fn delete_recipe(iid: &str, db: &State<MongoDB>) -> ApiResult<bool> {
    let id = parse_object_id(iid)?;
    let data = db.delete_recipe(&id).await?;

    ok(data)
}
