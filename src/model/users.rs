use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "chef")]
    Chef,
    #[serde(rename = "staff")]
    Staff,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub login_key: String,
    pub email: String,
    pub restaurant_id: ObjectId,
    pub user_type: UserType,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub _id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub restaurant_id: String,
    pub user_type: UserType,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl Into<UserResponse> for User {
    fn into(self) -> UserResponse {
        UserResponse {
            _id: self._id.unwrap().to_hex(),
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            restaurant_id: self.restaurant_id.to_hex(),
            user_type: self.user_type,
            active: self.active,
        }
    }
}
