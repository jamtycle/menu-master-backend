use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::product_order::{ProductOrder, ProductOrderRequest},
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
) -> Json<APIResponse<Option<Vec<ProductOrder>>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_restaurant_orders(id),
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

#[post("/", format = "application/json", data = "<info>")]
async fn create_product_order(
    info: Json<ProductOrderRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let product_order = db.create_product_order(&info.0);
    let message = if product_order.is_some() {
        "Product Order created."
    } else {
        "Product Order was not created."
    }
    .to_string();
    Json(APIResponse {
        code: 200,
        data: product_order,
        message,
    })
}

#[delete("/<pid>")]
async fn delete_product_order(pid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(pid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.delete_product_order(&id),
            message: "Product Order deleted.".to_string(),
        }),
        Err(ex) => {
            println!("{:?}", ex);
            return Json(APIResponse {
                code: 500,
                data: false,
                message: "ID bad format.".to_string(),
            });
        }
    }
}
