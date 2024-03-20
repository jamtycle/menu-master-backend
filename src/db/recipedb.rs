use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::{
        mongo_tables::Tables,
        recipe::{Recipe, RecipeRequest},
    },
};

impl MongoDB {
    pub fn get_all_recipes(&self) -> Option<Vec<Recipe>> {
        self.find(Tables::Recipes.value(), doc! {}, None)
    }

    pub fn get_recipe(&self, rid: &ObjectId) -> Option<Recipe> {
        self.find_one(Tables::Recipes.value(), doc! { "_id": rid.clone() }, None)
    }

    pub fn create_recipe(&self, recipe: &RecipeRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(recipe) {
            Some(ndoc) => self.create_one::<Recipe>(Tables::Recipes.value(), ndoc, None),
            None => None,
        }
    }

    pub fn update_recipe(&self, _id: &ObjectId, recipe: &RecipeRequest) -> bool {
        let id = _id.clone();
        match MongoDB::doc_from(recipe) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<Recipe>(Tables::Recipes.value(), doc! { "_id": id }, doc, None)
            }
            None => false,
        }
    }

    pub fn delete_recipe(&self, _id: &ObjectId) -> bool {
        self.delete_one::<Recipe>(Tables::Recipes.value(), doc! { "_id": _id.clone() }, None)
    }
}
