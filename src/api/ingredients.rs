use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::ingredients::{IngredientRequest, IngredientResponse},
};

use super::response::APIResponse;

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
async fn get_all_ingredients(
    rid: &str,
    db: &State<MongoDB>,
) -> Json<APIResponse<Vec<IngredientResponse>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse::new_success_nm(db.get_all_ingredients(&id))),
        Err(ex) => Json(APIResponse::new_error(ex.to_string().as_str())),
    }
}

#[post("/", format = "application/json", data = "<product>")]
async fn create_ingredient(
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<String>> {
    Json(APIResponse::new_success_nm(
        db.create_ingredient(&product.into_inner())
            .map(|x| x.to_hex()),
    ))
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
        return Json(APIResponse::new_error("bad id format"));
    }
    Json(APIResponse::new_success_nm(db.get_ingredient(
        &ingredient_id.unwrap(),
        &restaurant_id.unwrap(),
    )))
}

#[put("/<id>", format = "application/json", data = "<product>")]
async fn update_ingredient(
    id: &str,
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::from_str(id) {
        Ok(ingredient_id) => Json(APIResponse::new_success_nm(
            db.update_ingredient(&ingredient_id, &product.into_inner()),
        )),
        Err(ex) => Json(APIResponse::new_error(ex.to_string().as_str())),
    }
}

#[delete("/<rid>?<id>")]
async fn delete_ingredient(id: &str, rid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    let product_id = ObjectId::from_str(id);
    let restaurant_id = ObjectId::from_str(rid);
    if product_id.is_err() || restaurant_id.is_err() {
        return Json(APIResponse::new_error("bad id format"));
    }

    Json(APIResponse::new_success_nm(db.delete_ingredient(
        &product_id.unwrap(),
        &restaurant_id.unwrap(),
    )))
}
