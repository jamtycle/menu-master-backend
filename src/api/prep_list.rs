use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::prep_list::{PrepList, PrepListRequest},
};

use super::response::APIResponse;

#[get("/")]
pub async fn get_all_prep_list(_db: &State<MongoDB>) -> Json<Option<Vec<PrepList>>> {
    Json(_db.get_all_prep_list())
}

#[get("/<iid>")]
pub async fn get_prep_list(iid: &str, _db: &State<MongoDB>) -> Json<APIResponse<Option<PrepList>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: _db.get_prep_list(&id),
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
pub async fn create_prep_list(
    info: Json<PrepListRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let prep_list = _db.create_prep_list(&info.0);
    let message = if prep_list.is_some() {
        "Inventory created."
    } else {
        "Inventory was not created."
    }
    .to_string();
    let response = APIResponse {
        code: 200,
        data: prep_list,
        message,
    };
    Json(response)
}

#[put("/<iid>", format = "application/json", data = "<info>")]
pub async fn update_prep_list(
    iid: &str,
    info: Json<PrepListRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = _db.update_prep_list(&id, &info.0);
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
pub async fn delete_prep_list(iid: &str, _db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = _db.delete_prep_list(&id);
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
