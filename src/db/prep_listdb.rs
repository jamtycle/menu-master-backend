use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::{
        mongo_tables::Tables,
        prep_list::{PrepList, PrepListRequest},
    },
};

impl MongoDB {
    pub fn get_all_prep_list(&self) -> Option<Vec<PrepList>> {
        self.find(Tables::PrepLists.value(), doc! {}, None)
    }

    pub fn get_prep_list(&self, _pid: &ObjectId) -> Option<PrepList> {
        self.find_one(
            Tables::PrepLists.value(),
            doc! { "_id": _pid.clone() },
            None,
        )
    }

    pub fn create_prep_list(&self, _prep_list: &PrepListRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_prep_list) {
            Some(ndoc) => self.create_one::<PrepList>(Tables::PrepLists.value(), ndoc, None),
            None => None,
        }
    }

    pub fn update_prep_list(&self, _id: &ObjectId, _prep_list: &PrepListRequest) -> bool {
        let id = _id.clone();
        match MongoDB::doc_from(_prep_list) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<PrepList>(
                    Tables::PrepLists.value(),
                    doc! { "_id": id },
                    doc,
                    None,
                )
            }
            None => false,
        }
    }

    pub fn delete_prep_list(&self, _id: &ObjectId) -> bool {
        self.delete_one(Tables::PrepLists.value(), doc! { "_id": _id.clone() }, None)
    }
}
