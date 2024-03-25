use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOrder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub ingredient_id: ObjectId,
    pub restaurant_id: ObjectId,
    pub quantity: f64,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOrderRequest {
    pub restaurant_id: ObjectId,
    pub ingredient_id: ObjectId,
    pub quantity: f64,
    pub price: f64,
}
