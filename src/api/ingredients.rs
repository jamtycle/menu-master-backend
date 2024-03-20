use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{db::mongodb::MongoDB, model::product::Product};

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

#[get("/")]
async fn get_all_ingredients(_db: &State<MongoDB>) -> Json<APIResponse<Option<Vec<Product>>>> {
    let products = _db.get_all_products();
    let message = if let Some(ref products) = products {
        if !products.is_empty() {   
            "Products retrieved successfully."
        } else {
            "No products found."
        }
    } else {
        "Error retrieving products."
    }
    .to_string();

    let response = APIResponse {
        code: 200,
        data: products,
        message,
    };

    Json(response)
}

#[post("/", format = "application/json", data = "<product>")]
async fn create_ingredient(
    product: Json<Product>,
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
) -> Json<APIResponse<Option<Product>>> {
    let ingredient_id = ObjectId::from_str(&id).ok();
    let restaurant_id = ObjectId::from_str(&rid).ok();
    let product = db.get_ingredient(&ingredient_id.unwrap(), &restaurant_id.unwrap());

    let message = if product.is_some() {
        "Product retrieved successfully."
    } else {
        "Product not found."
    }
    .to_string();

    let response: APIResponse<Option<Product>> = APIResponse {
        code: 200,
        data: product,
        message,
    };

    Json(response)
}

#[put("/<id>", format = "application/json", data = "<product>")]
async fn update_ingredient(
    id: String,
    product: Json<Product>,
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
