use mongodb::bson::{doc, oid::ObjectId};

use crate::model::ingredients::{Ingredient, IngredientRequest, IngredientResponse};

use super::{
    mongo_tables::Tables,
    mongodb::{MongoDB, MongoDBResult},
};

impl MongoDB {
    pub async fn get_all_ingredients(
        &self,
        _rid: &ObjectId,
    ) -> MongoDBResult<Vec<IngredientResponse>> {
        let info = self
            .find_res::<Ingredient>(
                Tables::Ingredients.value(),
                doc! {
                    "restaurant_id": _rid.clone()
                },
                None,
            )
            .await?;

        Ok(info.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_ingredient(
        &self,
        _id: &ObjectId,
        _rid: &ObjectId,
    ) -> MongoDBResult<IngredientResponse> {
        let info = self
            .find_one_res::<Ingredient>(
                Tables::Ingredients.value(),
                doc! { "_id": _id.clone(), "restaurant_id": _rid.clone() },
                None,
            )
            .await?;

        Ok(info.into())
    }

    pub async fn create_ingredient(
        &self,
        _ingredient: &IngredientRequest,
    ) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(_ingredient)?;

        let data = self
            .create_one::<Ingredient>(Tables::Ingredients.value(), doc, None)
            .await?;

        Ok(data)
    }

    pub async fn update_ingredient(
        &self,
        _id: &ObjectId,
        _ingredient: &IngredientRequest,
    ) -> MongoDBResult<bool> {
        let id = _id.clone();
        let doc = MongoDB::doc_from(_ingredient)?;
        let data = self
            .update_one::<Ingredient>(Tables::Ingredients.value(), doc! { "_id": id }, doc, None)
            .await?;

        Ok(data)
    }

    pub async fn delete_ingredient(&self, _id: &ObjectId, _rid: &ObjectId) -> MongoDBResult<bool> {
        let data = self
            .delete_one(
                Tables::Ingredients.value(),
                doc! { "_id": _id.clone(), "restaurant_id": _rid.clone() },
                None,
            )
            .await?;

        Ok(data)
    }
}
