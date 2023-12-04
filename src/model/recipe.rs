use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::product::Product;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub recipe_name: String,
    pub ingredients: Vec<Product>,
    pub steps: Vec<String>,
}