use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{Inventory, InventoryRequest},
};

use super::response::APIResponse;

pub fn inventory_routes() -> Vec<rocket::Route> {
    routes![
        get_all_inventory,
        get_inventory,
        create_inventory,
        update_inventory,
    ]
}

#[get("/")]
async fn get_all_inventory(db: &State<MongoDB>) -> Json<Option<Vec<Inventory>>> {
    Json(db.get_all_inventory())
}

#[get("/<iid>")]
async fn get_inventory(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<Option<Inventory>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_inventory(&id),
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
async fn create_inventory(
    info: Json<InventoryRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let inventory = db.create_inventory(&info.0);
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
async fn update_inventory(
    iid: &str,
    info: Json<InventoryRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.update_inventory(&info.0);
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
