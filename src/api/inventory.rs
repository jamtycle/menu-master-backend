use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{InventoryRequest, InventoryResponse},
};

use super::{
    response::{ok, ApiResult},
    utils::parse_object_id,
};

pub fn inventory_routes() -> Vec<rocket::Route> {
    routes![
        get_all_inventory,
        get_inventory,
        create_inventory,
        update_inventory,
    ]
}

#[get("/")]
async fn get_all_inventory(db: &State<MongoDB>) -> ApiResult<Vec<InventoryResponse>> {
    ok(db.get_all_inventory().await?)
}

#[get("/<iid>")]
async fn get_inventory(iid: &str, db: &State<MongoDB>) -> ApiResult<InventoryResponse> {
    let id = parse_object_id(iid)?;

    ok(db.get_inventory(&id).await?)
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
