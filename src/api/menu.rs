use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::menu::{MenuRequest, MenuResponse},
};

use super::{
    response::{ok, ApiResult},
    utils::parse_object_id,
};

pub fn menu_routes() -> Vec<rocket::Route> {
    routes![get_menu, update_menu]
}

#[get("/<rid>")]
async fn get_menu(rid: &str, db: &State<MongoDB>) -> ApiResult<Vec<MenuResponse>> {
    let id = parse_object_id(rid)?;
    ok(db.get_menu(&id).await?)
}

#[post("/<mid>", format = "application/json", data = "<info>")]
async fn update_menu(mid: &str, info: Json<MenuRequest>, db: &State<MongoDB>) -> ApiResult<bool> {
    let id = parse_object_id(mid)?;
    let data = db.update_menu(&id, &info.0).await?;

    ok(data)
}
