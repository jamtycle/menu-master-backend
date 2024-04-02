use mongodb::bson::{doc, oid::ObjectId};

use crate::model::menu::{Menu, MenuRequest, MenuResponse};

use super::{
    mongo_tables::Tables,
    mongodb::{MongoDB, MongoDBResult},
};

impl MongoDB {
    pub fn get_all_menus(&self) -> Option<Vec<Menu>> {
        self.find(Tables::Menus.value(), doc! {}, None)
    }

    pub fn get_menu(&self, rid: &ObjectId) -> Option<Vec<MenuResponse>> {
        self.find::<Menu>(
            Tables::Menus.value(),
            doc! { "restaurant_id": rid.clone() },
            None,
        )
        .map(|m| m.into_iter().map(|x| x.into()).collect())
    }

    pub async fn create_menu(&self, menu: &MenuRequest) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(menu)?;
        let data = self
            .create_one::<Menu>(Tables::Menus.value(), doc, None)
            .await?;

        Ok(data)
    }

    pub async fn update_menu(&self, _id: &ObjectId, menu: &MenuRequest) -> MongoDBResult<bool> {
        let id = _id.clone();
        let doc = MongoDB::doc_from(menu)?;
        let data = self
            .update_one::<Menu>(Tables::Menus.value(), doc! { "_id": id }, doc, None)
            .await?;

        Ok(data)
    }

    pub async fn delete_menu(&self, _id: &ObjectId) -> MongoDBResult<bool> {
        let data = self
            .delete_one(Tables::Menus.value(), doc! { "_id": _id.clone() }, None)
            .await?;
        
        Ok(data)
    }
}
