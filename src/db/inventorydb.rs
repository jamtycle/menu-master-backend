use mongodb::{
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
};

use crate::{
    db::mongodb::MongoDB,
    model::{
        inventory::{Inventory, InventoryRequest},
        mongo_tables::Tables,
    },
};

impl MongoDB {
    pub fn get_all_inventory(&self) -> Option<Vec<Inventory>> {
        self.find(Tables::Inventory.value(), doc! {}, None)
    }

    pub fn get_inventory(&self, _ingredient_id: &ObjectId) -> Option<Inventory> {
        self.find_one(
            Tables::Inventory.value(),
            doc! { "ingredient_id": _ingredient_id.clone() },
            None,
        )
    }

    pub fn create_inventory(&self, _inventory: &InventoryRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_inventory) {
            Some(ndoc) => self.create_one::<Inventory>(Tables::Inventory.value(), ndoc, None),
            None => None,
        }
    }

    pub fn update_inventory(&self, _inventory: &InventoryRequest) -> bool {
        self.update_one::<Inventory>(
            Tables::Inventory.value(),
            doc! { "ingredient_id": _inventory.ingredient_id.clone() },
            doc! { "$inc": doc! { "stock": _inventory.stock } },
            UpdateOptions::builder().upsert(true).build(),
        )
    }

    pub fn delete_inventory(&self, _id: &ObjectId) -> bool {
        self.delete_one(Tables::Inventory.value(), doc! { "_id": _id.clone() }, None)
    }
}
