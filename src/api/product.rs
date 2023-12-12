use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::user::{LoginRequest, User},
    model::product::Product,
};

use super::response::APIResponse;

#[get("/")]
pub async fn get_users(_db: &State<MongoDB>) -> Json<Option<Vec<User>>> {
    Json(_db.get_users())
}

#[post("/login", format = "application/json", data = "<info>")]
pub async fn login_user(info: Json<LoginRequest>, _db: &State<MongoDB>) -> Json<APIResponse<Option<User>>> {
    let user = _db.login_user(&info.username, &info.password);
    let message = if user.is_some() { "User login success." } else { "Incorrect Username or Password." }.to_string();
    let response = APIResponse {
        code: 200,
        data: user,
        message,
    };
    Json(response)
}

#[post("/register", format = "application/json", data = "<info>")]
pub async fn register_user(info: Json<User>, _db: &State<MongoDB>) -> Json<APIResponse<Option<ObjectId>>> {
    let uid = _db.register_user(&info.0);
    let message = if uid.is_some() { "User registered successfully." } else { "User register error." }.to_string();
    let response = APIResponse {
        code: 200,
        data: uid,
        message,
    };
    Json(response)
}

#[get("/products")]
pub async fn get_all_products(_db: &State<MongoDB>) -> Json<APIResponse<Option<Vec<Product>>>> {
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

#[post("/products", format = "application/json", data = "<product>")]
pub async fn create_product(product: Json<Product>, _db: &State<MongoDB>) -> Json<APIResponse<Option<ObjectId>>> {
    let product_id = _db.create_product(&product.into_inner());
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

#[get("/products/<id>")]
pub async fn get_product_by_id(id: String, _db: &State<MongoDB>) -> Json<APIResponse<Option<Product>>> {
    let product_id = ObjectId::from_str(&id).ok();
    let product = _db.get_product(&product_id.unwrap());

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

#[put("/products", format = "application/json", data = "<product>")]
pub async fn update_product(product: Json<Product>, _db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    let success = _db.update_product(&product.into_inner());
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

#[delete("/products/<id>")]
pub async fn delete_product_by_id(id: String, _db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    let product_id = ObjectId::from_str(&id).ok();
    let success = _db.delete_product(&product_id.unwrap());

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