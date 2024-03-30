use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeCheck {
    pub recipe: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepList {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub user_id: ObjectId,
    pub recipe_checklist: Vec<RecipeCheck>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepListResponse {
    pub _id: String,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub user_id: String,
    pub recipe_checklist: Vec<RecipeCheck>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepListRequest {
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub user_id: ObjectId,
    pub recipe_checklist: Vec<RecipeCheck>,
}

impl Into<PrepListResponse> for PrepList {
    fn into(self) -> PrepListResponse {
        PrepListResponse {
            _id: self._id.unwrap().to_hex(),
            date: self.date,
            recipe_checklist: self.recipe_checklist,
            user_id: self.user_id.to_hex(),
        }
    }
}
