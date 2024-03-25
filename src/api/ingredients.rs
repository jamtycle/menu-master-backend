use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::ingredients::{Ingredient, IngredientRequest},
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
) -> Json<APIResponse<Option<Vec<Ingredient>>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_all_ingredients(&id),
            message: "".to_string(),
        }),
        Err(ex) => {
            println!("{:?}", ex);
            return Json(APIResponse {
                code: 500,
                data: None,
                message: "ID bad format.".to_string(),
            });
        }
    }
}

#[post("/", format = "application/json", data = "<product>")]
async fn create_ingredient(
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let product_id = db.create_ingredient(&product.into_inner());
    let message = if product_id.is_some() {
        "Product created successfully."
    } else {
        "Error creating the product."
    }
    .to_string();

    let response = APIResponse {
        code: 200,
        data: product_id,
        message,
    };

    Json(response)
}

#[get("/<rid>?<id>")]
async fn get_ingredint_by_id(
    id: String,
    rid: String,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<Ingredient>>> {
    let ingredient_id = ObjectId::from_str(&id).ok();
    let restaurant_id = ObjectId::from_str(&rid).ok();
    let product = db.get_ingredient(&ingredient_id.unwrap(), &restaurant_id.unwrap());

    let message = if product.is_some() {
        "Product retrieved successfully."
    } else {
        "Product not found."
    }
    .to_string();

    let response: APIResponse<Option<Ingredient>> = APIResponse {
        code: 200,
        data: product,
        message,
    };

    Json(response)
}

#[put("/<id>", format = "application/json", data = "<product>")]
async fn update_ingredient(
    id: String,
    product: Json<IngredientRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    let ingredient_id = ObjectId::from_str(&id).ok();
    let success = db.update_ingredient(&ingredient_id.unwrap(), &product.into_inner());
    let message = if success {
        "Product updated successfully."
    } else {
        "Error updating the product."
    }
    .to_string();

    let response = APIResponse {
        code: 200,
        data: success,
        message,
    };

    Json(response)
}

#[delete("/<rid>?<id>")]
async fn delete_ingredient(
    id: String,
    rid: String,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    let product_id = ObjectId::from_str(&id).ok();
    let restaurant_id = ObjectId::from_str(&rid).ok();
    let success = db.delete_ingredient(&product_id.unwrap(), &restaurant_id.unwrap());

    let message = if success {
        "Product deleted successfully."
    } else {
        "Error deleting the product."
    }
    .to_string();

    let response = APIResponse {
        code: 200,
        data: success,
        message,
    };

    Json(response)
}
