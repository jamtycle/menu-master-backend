use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{InventoryRequest, InventoryResponse},
};

use super::response::{ok, APIResponse, ApiResult};

pub fn inventory_routes() -> Vec<rocket::Route> {
    routes![
        get_all_inventory,
        get_inventory,
        create_inventory,
        update_inventory,
    ]
}

#[get("/")]
async fn get_all_inventory(db: &State<MongoDB>) -> Json<APIResponse<Vec<InventoryResponse>>> {
    Json(APIResponse::new_success_nm(db.get_all_inventory()))
}

#[get("/<iid>")]
async fn get_inventory(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<InventoryResponse>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse::new_success_nm(db.get_inventory(&id))),
        Err(ex) => Json(APIResponse::new_error(
            Status::BadRequest,
            ex.to_string().as_str(),
        )),
    }
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_inventory(info: Json<InventoryRequest>, db: &State<MongoDB>) -> ApiResult<String> {
    let inventory = db.create_inventory(&info.0).await?;
    ok(inventory.to_hex())
}

#[put("/", format = "application/json", data = "<info>")]
async fn update_inventory(info: Json<InventoryRequest>, db: &State<MongoDB>) -> ApiResult<bool> {
    let data = db.update_inventory(&info.0).await?;
    ok(data)
}
