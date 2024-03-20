use crate::model::{mongo_tables::Tables, restaurants::Restaurant};

use super::mongodb::MongoDB;

impl MongoDB {
    pub fn get_all_restaurants(&self) -> Option<Vec<Restaurant>> {
        self.find(Tables::Restaurants.value(), doc! {}, None)
    }
}