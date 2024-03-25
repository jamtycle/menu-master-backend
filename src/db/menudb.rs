use mongodb::bson::{doc, oid::ObjectId};

use crate::model::{
    menu::{Menu, MenuRequest},
    mongo_tables::Tables,
};

use super::mongodb::MongoDB;

impl MongoDB {
    pub fn get_all_menus(&self) -> Option<Vec<Menu>> {
        self.find(Tables::Menus.value(), doc! {}, None)
    }

    pub fn get_menu(&self, rid: &ObjectId) -> Option<Menu> {
        self.find_one(Tables::Menus.value(), doc! { "_id": rid.clone() }, None)
    }

    pub fn create_menu(&self, menu: &MenuRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(menu) {
            Some(ndoc) => self.create_one::<Menu>(Tables::Menus.value(), ndoc, None),
            None => None,
        }
    }

    pub fn update_menu(&self, _id: &ObjectId, menu: &MenuRequest) -> bool {
        let id = _id.clone();
        match MongoDB::doc_from(menu) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<Menu>(Tables::Menus.value(), doc! { "_id": id }, doc, None)
            }
            None => false,
        }
    }

    pub fn delete_menu(&self, _id: &ObjectId) -> bool {
        self.delete_one(Tables::Menus.value(), doc! { "_id": _id.clone() }, None)
    }
}
