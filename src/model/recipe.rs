use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub recipe_name: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeRequest {
    pub recipe_name: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
}
