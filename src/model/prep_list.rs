use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

use super::recipe::Recipe;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeCheck {
    pub recipe: Recipe,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepList {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(with="mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub recipe_checklist: Vec<RecipeCheck>,
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepListRequest {
    #[serde(with="mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub recipe_checklist: Vec<RecipeCheck>,
    pub user_id: ObjectId,
}