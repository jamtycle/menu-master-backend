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
pub struct RecipeIngredientResponse {
    cost: f64,
    ingredient_id: String,
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
pub struct RecipeResponse {
    pub _id: String,
    pub name: String,
    pub restaurant_id: String,
    pub image_src: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<RecipeIngredientResponse>,
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

impl Into<RecipeResponse> for Recipe {
    fn into(self) -> RecipeResponse {
        RecipeResponse {
            _id: self._id.unwrap().to_hex(),
            name: self.name,
            restaurant_id: self.restaurant_id.to_hex(),
            image_src: self.image_src,
            steps: self.steps,
            ingredients: self
                .ingredients
                .into_iter()
                .map(|x| RecipeIngredientResponse {
                    cost: x.cost,
                    ingredient_id: x.ingredient_id.to_hex(),
                    measure_unit: x.measure_unit,
                    quantity: x.quantity,
                })
                .collect(),
            prep_time_min: self.prep_time_min,
            recipe_type: self.recipe_type,
            active: self.active,
        }
    }
}
