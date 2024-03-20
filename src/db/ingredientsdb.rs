use mongodb::bson::oid::ObjectId;

use crate::model::{
    ingredients::{Ingredient, IngredientRequest},
    mongo_tables::Tables,
};

use super::mongodb::MongoDB;

impl MongoDB {
    pub fn get_all_ingredients(&self, _rid: &ObjectId) -> Option<Vec<Ingredient>> {
        self.find(
            Tables::Ingredients.value(),
            doc! {
                "restaurant_id": _rid.clone()
            },
            None,
        )
    }

    pub fn get_ingredient(&self, _id: &ObjectId, _rid: &ObjectId) -> Option<Ingredient> {
        self.find_one(
            Tables::Ingredients.value(),
            doc! { "_id": _pid.clone(), "restaurant_id": _rid.clone() },
            None,
        )
    }

    pub fn create_ingredient(&self, _ingredient: &IngredientRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_ingredient) {
            Some(ndoc) => self.create_one::<Ingredient>(Tables::Ingredients.value(), ndoc, None),
            None => None,
        }
    }

    pub fn update_ingredient(&self, _id: &ObjectId, _ingredient: &IngredientRequest) -> bool {
        let id = _id.clone();

        match MongoDB::doc_from(_ingredient) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<Ingredient>(
                    Tables::Ingredients.value(),
                    doc! { "_id": id },
                    doc,
                    None,
                )
            }
            None => false,
        }
    }

    pub fn delete_ingredient(&self, _id: &ObjectId, _rid: &ObjectId) -> bool {
        self.delete_one(
            Tables::Ingredients.value(),
            doc! { "_id": _id.clone(), "restaurant_id": _rid.clone() },
            None,
        )
    }
}
