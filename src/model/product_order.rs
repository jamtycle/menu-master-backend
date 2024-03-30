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
pub struct ProductOrderResponse {
    pub _id: String,
    pub ingredient_id: String,
    pub restaurant_id: String,
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

impl Into<ProductOrderResponse> for ProductOrder {
    fn into(self) -> ProductOrderResponse {
        ProductOrderResponse {
            _id: self._id.unwrap().to_hex(),
            ingredient_id: self.ingredient_id.to_hex(),
            restaurant_id: self.restaurant_id.to_hex(),
            quantity: self.quantity,
            price: self.price,
        }
    }
}
