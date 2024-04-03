use mongodb::{
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{Inventory, InventoryRequest, InventoryResponse},
};

use super::{mongo_tables::Tables, mongodb::MongoDBResult};

impl MongoDB {
    pub async fn get_all_inventory(&self) -> MongoDBResult<Vec<InventoryResponse>> {
        let data = self
            .find_res::<Inventory>(Tables::Inventory.value(), doc! {}, None)
            .await?;

        Ok(data.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_inventory(
        &self,
        _ingredient_id: &ObjectId,
    ) -> MongoDBResult<InventoryResponse> {
        let data = self
            .find_one_res::<Inventory>(
                Tables::Inventory.value(),
                doc! { "ingredient_id": _ingredient_id.clone() },
                None,
            )
            .await?;

        Ok(data.into())
    }

    pub async fn create_inventory(&self, _inventory: &InventoryRequest) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(_inventory)?;
        let data = self
            .create_one::<Inventory>(Tables::Inventory.value(), doc, None)
            .await?;

        Ok(data)
    }

    pub async fn update_inventory(&self, _inventory: &InventoryRequest) -> MongoDBResult<bool> {
        let data = self
            .update_one::<Inventory>(
                Tables::Inventory.value(),
                doc! { "ingredient_id": _inventory.ingredient_id.clone() },
                doc! { "$inc": doc! { "stock": _inventory.stock } },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(data)
    }

    pub async fn delete_inventory(&self, _id: &ObjectId) -> MongoDBResult<bool> {
        let data = self
            .delete_one(Tables::Inventory.value(), doc! { "_id": _id.clone() }, None)
            .await?;

        Ok(data)
    }
}
