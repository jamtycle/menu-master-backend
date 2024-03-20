use mongodb::{
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
};

use crate::{
    db::mongodb::MongoDB,
    model::inventory::{Inventory, InventoryRequest},
};

impl MongoDB {
    pub fn get_all_inventory(&self) -> Option<Vec<Inventory>> {
        self.find("inventory", doc! {}, None)
    }

    pub fn get_inventory(&self, _pid: &ObjectId) -> Option<Inventory> {
        self.find_one("inventory", doc! { "_id": _pid.clone() }, None)
    }

    pub fn create_inventory(&self, _inventory: &InventoryRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_inventory) {
            Some(ndoc) => self.create_one::<Inventory>("inventory", ndoc, None),
            None => None,
        }
    }

    pub fn update_inventory(&self, _id: &ObjectId, _inventory: &InventoryRequest) -> bool {
        let id = _id.clone();

        match MongoDB::doc_from(_inventory) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<Inventory>(
                    "inventory",
                    doc! { "_id": id },
                    doc,
                    UpdateOptions::builder().upsert(true).build(),
                )
            }
            None => false,
        }
    }

    pub fn delete_inventory(&self, _id: &ObjectId) -> bool {
        self.delete_one::<Inventory>("inventory", doc! { "_id": _id.clone() }, None)
    }
}
