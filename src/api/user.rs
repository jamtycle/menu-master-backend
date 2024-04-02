use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::users::{LoginRequest, User, UserResponse},
};

use super::response::{ok, APIResponse, ApiResult};

pub fn user_routes() -> Vec<rocket::Route> {
    routes![get_users, login_user, register_user]
}

#[get("/")]
async fn get_users(db: &State<MongoDB>) -> Json<APIResponse<Vec<UserResponse>>> {
    Json(APIResponse::new_success_nm(
        db.get_users()
            .map(|u| u.into_iter().map(|user| user.into()).collect()),
    ))
}

#[post("/login", format = "application/json", data = "<info>")]
async fn login_user(
    info: Json<LoginRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<UserResponse>> {
    Json(APIResponse::new_success_nm(
        db.login_user(&info.username, &info.password),
    ))
}

#[post("/register", format = "application/json", data = "<info>")]
async fn register_user(info: Json<User>, db: &State<MongoDB>) -> ApiResult<String> {
    let data = db.register_user(&info.0).await?;

    ok(data.to_hex())
}
