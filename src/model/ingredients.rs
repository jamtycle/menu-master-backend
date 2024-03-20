#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub restaurant_id: ObjectId,
    pub price: f64,
    pub unit: String,
    pub item_par: f64,
    pub category: String,
    pub supplier: String,
    pub season: String,
    pub pack_size: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngredientRequest {
    pub name: String,
    pub restaurant_id: ObjectId,
    pub price: f64,
    pub unit: String,
    pub item_par: f64,
    pub category: String,
    pub supplier: String,
    pub season: String,
    pub pack_size: String,
    pub notes: String,
}