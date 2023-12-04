use mongodb::bson::{oid::ObjectId, Decimal128};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub product_id: Decimal128,
    pub quantity: i32,
}