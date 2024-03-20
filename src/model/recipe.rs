use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIngredient {
    cost: f64,
    ingredient_id: ObjectId,
    measure_unit: String,
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub restaurant_id: ObjectId,
    pub image_src: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
    pub prep_time_min: f64,
    pub recipe_type: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeRequest {
    pub name: String,
    pub restaurant_id: ObjectId,
    pub image_src: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
    pub prep_time_min: f64,
    pub recipe_type: String,
    pub active: bool,
}
