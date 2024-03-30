use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::product_order::{ProductOrderRequest, ProductOrderResponse},
};

use super::response::APIResponse;

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
        Err(ex) => Json(APIResponse::new_error(ex.to_string().as_str())),
    }
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_product_order(
    info: Json<ProductOrderRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<String>> {
    let product_order = db.create_product_order(&info.0);
    Json(APIResponse::new_success_nm(
        product_order.map(|id| id.to_hex()),
    ))
}

#[delete("/<pid>")]
async fn delete_product_order(pid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    let id = ObjectId::parse_str(pid).unwrap();
    Json(APIResponse::new_success(
        db.delete_product_order(&id),
        "Product Order deleted.",
    ))
}
