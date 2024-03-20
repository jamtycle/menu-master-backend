use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{Inventory, InventoryRequest},
};

use super::response::APIResponse;

#[get("/")]
pub async fn get_all_inventory(_db: &State<MongoDB>) -> Json<Option<Vec<Inventory>>> {
    Json(_db.get_all_inventory())
}

#[get("/<iid>")]
pub async fn get_inventory(
    iid: &str,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<Inventory>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: _db.get_inventory(&id),
            message: "".to_string(),
        }),
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: None,
                message: "ID bad format.".to_string(),
            })
        }
    }
}

#[post("/", format = "application/json", data = "<info>")]
pub async fn create_inventory(
    info: Json<InventoryRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let inventory = _db.create_inventory(&info.0);
    let message = if inventory.is_some() {
        "Inventory created."
    } else {
        "Inventory was not created."
    }
    .to_string();
    let response = APIResponse {
        code: 200,
        data: inventory,
        message,
    };
    Json(response)
}

#[put("/<iid>", format = "application/json", data = "<info>")]
pub async fn update_inventory(
    iid: &str,
    info: Json<InventoryRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.update_inventory(&id, &info.0);
            let message = if update {
                "Inventory updated."
            } else {
                "Error while updating."
            }
            .to_string();
            let response = APIResponse {
                code: 200,
                data: update,
                message,
            };
            Json(response)
        }
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: false,
                message: "ID bad format".to_string(),
            })
        }
    }
}

#[delete("/<iid>")]
pub async fn delete_inventory(iid: &str, _db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = _db.delete_inventory(&id);
            let message = if update {
                "Inventory deleted."
            } else {
                "Error while deleting."
            }
            .to_string();
            let response = APIResponse {
                code: 200,
                data: update,
                message,
            };
            Json(response)
        }
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: false,
                message: "ID bad format".to_string(),
            })
        }
    }
}
