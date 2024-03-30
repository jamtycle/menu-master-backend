use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
    pub raw_material: bool,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngredientResponse {
    pub _id: String,
    pub name: String,
    pub restaurant_id: String,
    pub price: f64,
    pub unit: String,
    pub item_par: f64,
    pub category: String,
    pub supplier: String,
    pub season: String,
    pub pack_size: String,
    pub raw_material: bool,
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
    pub raw_material: bool,
    pub notes: String,
}

impl Into<IngredientResponse> for Ingredient {
    fn into(self) -> IngredientResponse {
        IngredientResponse {
            _id: self._id.unwrap().to_hex(),
            name: self.name,
            restaurant_id: self.restaurant_id.to_hex(),
            price: self.price,
            unit: self.unit,
            item_par: self.item_par,
            category: self.category,
            supplier: self.supplier,
            season: self.season,
            pack_size: self.pack_size,
            raw_material: self.raw_material,
            notes: self.notes,
        }
    }
}
 