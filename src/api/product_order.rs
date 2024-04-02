use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::product_order::{ProductOrderRequest, ProductOrderResponse},
};

use super::response::{ok, APIResponse, ApiResult};

pub fn product_order_routes() -> Vec<rocket::Route> {
    routes![
        get_restaurant_orders,
        create_product_order,
        delete_product_order
    ]
}

#[get("/<rid>")]
async fn get_restaurant_orders(
    rid: &str,
    db: &State<MongoDB>,
) -> Json<APIResponse<Vec<ProductOrderResponse>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse::new_success_nm(db.get_restaurant_orders(id))),
        Err(ex) => Json(APIResponse::new_error(
            Status::BadRequest,
            ex.to_string().as_str(),
        )),
    }
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_product_order(
    info: Json<ProductOrderRequest>,
    db: &State<MongoDB>,
) -> ApiResult<String> {
    let product_order = db.create_product_order(&info.0).await?;
    ok(product_order.to_hex())
}

#[delete("/<pid>")]
async fn delete_product_order(pid: &str, db: &State<MongoDB>) -> ApiResult<bool> {
    let id = ObjectId::parse_str(pid).unwrap();
    ok(db.delete_product_order(&id).await?)
}
