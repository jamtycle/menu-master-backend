use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Restaurant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    pub cuisine: String,
    pub profit_rate: f64,
    pub active: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestaurantResponse {
    pub _id: String,
    pub name: String,
    pub address: String,
    pub cuisine: String,
    pub profit_rate: f64,
    pub active: bool,
    pub deleted: bool,
}

impl Into<RestaurantResponse> for Restaurant {
    fn into(self) -> RestaurantResponse {
        RestaurantResponse {
            _id: self._id.unwrap().to_hex(),
            name: self.name,
            address: self.address,
            cuisine: self.cuisine,
            profit_rate: self.profit_rate,
            active: self.active,
            deleted: self.deleted,
        }
    }
}
