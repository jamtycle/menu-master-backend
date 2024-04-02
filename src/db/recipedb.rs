use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{Recipe, RecipeRequest, RecipeResponse},
};

use super::{mongo_tables::Tables, mongodb::MongoDBResult};

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

    pub async fn create_recipe(&self, recipe: &RecipeRequest) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(recipe)?;
        let data = self
            .create_one::<Recipe>(Tables::Recipes.value(), doc, None)
            .await?;

        Ok(data)
    }

    pub async fn update_recipe(
        &self,
        _id: &ObjectId,
        recipe: &RecipeRequest,
    ) -> MongoDBResult<bool> {
        let id = _id.clone();
        let doc = MongoDB::doc_from(recipe)?;
        let data = self
            .update_one::<Recipe>(Tables::Recipes.value(), doc! { "_id": id }, doc, None)
            .await?;

        Ok(data)
    }

    pub async fn delete_recipe(&self, _id: &ObjectId) -> MongoDBResult<bool> {
        let data = self
            .delete_one(Tables::Recipes.value(), doc! { "_id": _id.clone() }, None)
            .await?;

        Ok(data)
    }
}
