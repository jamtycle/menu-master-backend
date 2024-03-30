use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::menu::{MenuRequest, MenuResponse},
};

use super::response::APIResponse;

pub fn menu_routes() -> Vec<rocket::Route> {
    routes![get_menu, update_menu]
}

#[get("/<rid>")]
async fn get_menu(rid: &str, db: &State<MongoDB>) -> Json<APIResponse<Vec<MenuResponse>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse::new_success_nm(db.get_menu(&id))),
        Err(ex) => Json(APIResponse::new_error(ex.to_string().as_str())),
    }
}

#[post("/<mid>", format = "application/json", data = "<info>")]
async fn update_menu(
    mid: &str,
    info: Json<MenuRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(mid) {
        Ok(id) => Json(APIResponse::new_success_nm(db.update_menu(&id, &info.0))),
        Err(ex) => Json(APIResponse::new_error(ex.to_string().as_str())),
    }
}
