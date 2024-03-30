use mongodb::bson::{doc, oid::ObjectId};

use crate::model::ingredients::{Ingredient, IngredientRequest, IngredientResponse};

use super::{mongo_tables::Tables, mongodb::MongoDB};

impl MongoDB {
    pub fn get_all_ingredients(&self, _rid: &ObjectId) -> Option<Vec<IngredientResponse>> {
        self.find::<Ingredient>(
            Tables::Ingredients.value(),
            doc! {
                "restaurant_id": _rid.clone()
            },
            None,
        )
        .map(|d| d.into_iter().map(|x| x.into()).collect())
    }

    pub fn get_ingredient(&self, _id: &ObjectId, _rid: &ObjectId) -> Option<IngredientResponse> {
        self.find_one::<Ingredient>(
            Tables::Ingredients.value(),
            doc! { "_id": _id.clone(), "restaurant_id": _rid.clone() },
            None,
        )
        .map(|d| d.into())
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
