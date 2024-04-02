use mongodb::bson::{oid::ObjectId, Decimal128};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub ingredient_id: ObjectId,
    pub stock: Decimal128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub _id: String,
    pub ingredient_id: String,
    pub stock: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryRequest {
    pub ingredient_id: ObjectId,
    pub stock: f64,
}

impl Into<InventoryResponse> for Inventory {
    fn into(self) -> InventoryResponse {
        InventoryResponse {
            _id: self._id.unwrap().to_hex(),
            ingredient_id: self.ingredient_id.to_hex(),
            stock: self.stock.to_string().parse::<f64>().unwrap(),
        }
    }
}
