use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{Recipe, RecipeRequest},
};

impl MongoDB {
    pub fn get_all_recipes(&self) -> Option<Vec<Recipe>> {
        self.find("recipe", doc! {}, None)
    }

    pub fn get_recipe(&self, _pid: &ObjectId) -> Option<Recipe> {
        self.find_one("recipe", doc! { "_id": _pid.clone() }, None)
    }

    pub fn create_recipe(&self, _prep_list: &RecipeRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_prep_list) {
            Some(ndoc) => self.create_one::<Recipe>("recipe", ndoc, None),
            None => None,
        }
    }

    pub fn update_recipe(&self, _id: &ObjectId, _prep_list: &RecipeRequest) -> bool {
        let id = _id.clone();
        match MongoDB::doc_from(_prep_list) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<Recipe>("recipe", doc! { "_id": id }, doc, None)
            }
            None => false,
        }
    }

    pub fn delete_recipe(&self, _id: &ObjectId) -> bool {
        self.delete_one::<Recipe>("recipe", doc! { "_id": _id.clone() }, None)
    }
}
