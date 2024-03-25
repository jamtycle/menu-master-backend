use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::menu::{Menu, MenuRequest},
};

use super::response::APIResponse;

pub fn menu_routes() -> Vec<rocket::Route> {
    routes![get_menu, update_menu]
}

#[get("/<rid>")]
async fn get_menu(rid: &str, db: &State<MongoDB>) -> Json<APIResponse<Option<Menu>>> {
    match ObjectId::parse_str(rid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_menu(&id),
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

#[post("/<mid>", format = "application/json", data = "<info>")]
async fn update_menu(
    mid: &str,
    info: Json<MenuRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(mid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.update_menu(&id, &info.0),
            message: "".to_string(),
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
    // let menu = db.update_menu(&mid, &info.0);
    // let message = if menu.is_some() {
    //     "Menu created."
    // } else {
    //     "Menu was not created."
    // }
    // .to_string();
    // let response = APIResponse {
    //     code: 200,
    //     data: menu,
    //     message,
    // };
    // Json(response)
}
