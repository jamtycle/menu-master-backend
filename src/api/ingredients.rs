use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    api::utils::parse_object_id,
    db::mongodb::MongoDB,
    model::ingredients::{IngredientRequest, IngredientResponse},
};

use super::response::{ok, APIResponse, ApiResult};

pub fn ingredient_routes() -> Vec<rocket::Route> {
    routes![
        get_all_ingredients,
        create_ingredient,
        get_ingredint_by_id,
        update_ingredient,
        delete_ingredient
    ]
}

#[get("/<rid>")]
async fn get_all_ingredients(rid: &str, db: &State<MongoDB>) -> ApiResult<Vec<IngredientResponse>> {
    let id = parse_object_id(rid)?;
    let data = db.get_all_ingredients(&id).await?;

    ok(data)
}

#[get("/<rid>?<id>")]
async fn get_ingredint_by_id(
    id: &str,
    rid: &str,
    db: &State<MongoDB>,
) -> Json<APIResponse<IngredientResponse>> {
    let ingredient_id = ObjectId::from_str(id);
    let restaurant_id = ObjectId::from_str(rid);
    if ingredient_id.is_err() || restaurant_id.is_err() {
        return Json(APIResponse::new_error(Status::BadRequest, "bad id format"));
    }
    Json(APIResponse::new_success_nm(db.get_ingredient(
        &ingredient_id.unwrap(),
        &restaurant_id.unwrap(),
    )))
}

#[post("/", format = "application/json", data = "<product>")]
async fn create_ingredient(
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> ApiResult<String> {
    let data = db
        .create_ingredient(&product.into_inner())
        .await
        .map(|x| x.to_hex())?;

    ok(data)
}

#[put("/<id>", format = "application/json", data = "<product>")]
async fn update_ingredient(
    id: &str,
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> ApiResult<bool> {
    let id = parse_object_id(id)?;
    let data = db.update_ingredient(&id, &product.into_inner()).await?;

    ok(data)
}

#[delete("/<rid>?<id>")]
async fn delete_ingredient(id: &str, rid: &str, db: &State<MongoDB>) -> ApiResult<bool> {
    let product_id = parse_object_id(id)?;
    let restaurant_id = parse_object_id(rid)?;

    let data = db.delete_ingredient(&product_id, &restaurant_id).await?;

    ok(data)
}
