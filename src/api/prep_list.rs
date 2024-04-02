use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::prep_list::{PrepListRequest, PrepListResponse},
};

use super::{
    response::{ok, APIResponse, ApiResult},
    utils::parse_object_id,
};

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
async fn get_all_prep_list(db: &State<MongoDB>) -> Json<APIResponse<Vec<PrepListResponse>>> {
    Json(APIResponse::new_success_nm(db.get_all_prep_list()))
}

#[get("/<iid>")]
async fn get_prep_list(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<PrepListResponse>> {
    let id = ObjectId::parse_str(iid).unwrap();
    Json(APIResponse::new_success_nm(db.get_prep_list(&id)))
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_prep_list(info: Json<PrepListRequest>, db: &State<MongoDB>) -> ApiResult<String> {
    let info = db.create_prep_list(&info.0).await?;

    ok(info.to_hex())
}

#[put("/<iid>", format = "application/json", data = "<info>")]
async fn update_prep_list(
    iid: &str,
    info: Json<PrepListRequest>,
    db: &State<MongoDB>,
) -> ApiResult<bool> {
    let id = parse_object_id(iid)?;

    ok(db.update_prep_list(&id, &info.0).await?)
}

#[delete("/<iid>")]
async fn delete_prep_list(iid: &str, db: &State<MongoDB>) -> ApiResult<bool> {
    let id = parse_object_id(iid)?;

    ok(db.delete_prep_list(&id).await?)
}
