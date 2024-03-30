use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{Recipe, RecipeRequest, RecipeResponse},
};

use super::mongo_tables::Tables;

impl MongoDB {
    pub fn get_all_recipes(&self) -> Option<Vec<RecipeResponse>> {
        let recipes = self.find::<Recipe>(Tables::Recipes.value(), doc! {}, None);
        return recipes.map(|x| x.into_iter().map(|r| r.into()).collect());
    }

    pub fn get_recipe(&self, rid: &ObjectId) -> Option<RecipeResponse> {
        let recipe =
            self.find_one::<Recipe>(Tables::Recipes.value(), doc! { "_id": rid.clone() }, None);
        return recipe.map(|x| x.into());
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
        self.delete_one(Tables::Recipes.value(), doc! { "_id": _id.clone() }, None)
    }
}
