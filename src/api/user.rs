use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::users::{LoginRequest, User},
};

use super::response::APIResponse;

pub fn user_routes() -> Vec<rocket::Route> {
    routes![get_users, login_user, register_user]
}

#[get("/")]
async fn get_users(_db: &State<MongoDB>) -> Json<Option<Vec<User>>> {
    Json(_db.get_users())
}

#[post("/login", format = "application/json", data = "<info>")]
async fn login_user(
    info: Json<LoginRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<User>>> {
    let user = _db.login_user(&info.username, &info.password);
    let message = if user.is_some() {
        "User login success."
    } else {
        "Incorrect Username or Password."
    }
    .to_string();
    let response = APIResponse {
        code: 200,
        data: user,
        message,
    };
    Json(response)
}

#[post("/register", format = "application/json", data = "<info>")]
async fn register_user(
    info: Json<User>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let uid = _db.register_user(&info.0);
    let message = if uid.is_some() {
        "User registered successfully."
    } else {
        "User register error."
    }
    .to_string();
    let response = APIResponse {
        code: 200,
        data: uid,
        message,
    };
    Json(response)
}
