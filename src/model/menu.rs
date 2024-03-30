use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub recipe_id: ObjectId,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItemResponse {
    pub recipe_id: String,
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
pub struct MenuResponse {
    pub id: String,
    pub name: String,
    pub restaurant_id: String,
    pub food_profit: f64,
    pub items: Vec<MenuItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuRequest {
    pub name: String,
    pub restaurant_id: ObjectId,
    pub food_profit: f64,
    pub items: Vec<MenuItem>,
}

impl Into<MenuResponse> for Menu {
    fn into(self) -> MenuResponse {
        MenuResponse {
            id: self._id.unwrap().to_hex(),
            name: self.name,
            restaurant_id: self.restaurant_id.to_hex(),
            food_profit: self.food_profit,
            items: self
                .items
                .iter()
                .map(|i| MenuItemResponse {
                    recipe_id: i.recipe_id.to_hex(),
                    price: i.price,
                })
                .collect(),
        }
    }
}
