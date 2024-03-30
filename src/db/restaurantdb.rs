use mongodb::bson::doc;

use crate::model::restaurants::{Restaurant, RestaurantResponse};

use super::{mongo_tables::Tables, mongodb::MongoDB};

impl MongoDB {
    pub fn get_all_restaurants(&self) -> Option<Vec<RestaurantResponse>> {
        self.find::<Restaurant>(Tables::Restaurants.value(), doc! {}, None)
            .map(|x| x.into_iter().map(|r| r.into()).collect())
    }
}
