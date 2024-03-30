use mongodb::bson::{doc, oid::ObjectId};

use crate::model::menu::{Menu, MenuRequest, MenuResponse};

use super::{mongo_tables::Tables, mongodb::MongoDB};

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

    pub fn create_menu(&self, menu: &MenuRequest) -> Option<ObjectId> {
        let doc = MongoDB::doc_from(menu);
        return self.create_one::<Menu>(Tables::Menus.value(), doc.unwrap(), None);
    }

    pub fn update_menu(&self, _id: &ObjectId, menu: &MenuRequest) -> bool {
        let id = _id.clone();
        let doc = MongoDB::doc_from(menu);
        return self.update_one::<Menu>(
            Tables::Menus.value(),
            doc! { "_id": id },
            doc.unwrap(),
            None,
        );
    }

    pub fn delete_menu(&self, _id: &ObjectId) -> bool {
        self.delete_one(Tables::Menus.value(), doc! { "_id": _id.clone() }, None)
    }
}
