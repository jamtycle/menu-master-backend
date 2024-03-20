use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub recipe_id: ObjectId,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub restaurant_id: ObjectId,
    pub food_profit: f64,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuRequest {
    pub name: String,
    pub restaurant_id: ObjectId,
    pub food_profit: f64,
    pub items: Vec<MenuItem>,
}