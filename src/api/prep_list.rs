use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::prep_list::{PrepList, PrepListRequest},
};

use super::response::APIResponse;

pub fn prep_list_routes() -> Vec<rocket::Route> {
    routes![
        get_all_prep_list,
        get_prep_list,
        create_prep_list,
        update_prep_list,
        delete_prep_list
    ]
}

#[get("/")]
async fn get_all_prep_list(db: &State<MongoDB>) -> Json<APIResponse<Option<Vec<PrepList>>>> {
    Json(APIResponse {
        code: 200,
        data: db.get_all_prep_list(),
        message: "".to_string(),
    })
}

#[get("/<iid>")]
async fn get_prep_list(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<Option<PrepList>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_prep_list(&id),
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
async fn create_prep_list(
    info: Json<PrepListRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let prep_list = db.create_prep_list(&info.0);
    let message = if prep_list.is_some() {
        "Prep List created."
    } else {
        "Prep List was not created."
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
async fn update_prep_list(
    iid: &str,
    info: Json<PrepListRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.update_prep_list(&id, &info.0);
            let message = if update {
                "Prep List updated."
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
async fn delete_prep_list(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.delete_prep_list(&id);
            let message = if update {
                "Prep List deleted."
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
