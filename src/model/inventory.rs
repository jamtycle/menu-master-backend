use mongodb::bson::{oid::ObjectId, Decimal128};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub ingredient_id: ObjectId,
    pub stock: Decimal128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryRequest {
    pub ingredient_id: ObjectId,
    pub stock: Decimal128,
}
