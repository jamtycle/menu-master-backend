use mongodb::bson::{doc, oid::ObjectId};

use crate::{db::mongodb::MongoDB, model::prep_list::{PrepList, PrepListRequest}};

impl MongoDB {
    pub fn get_all_prep_list(&self) -> Option<Vec<PrepList>> {
        self.find("prep_list", doc! {}, None)
    }

    pub fn get_prep_list(&self, _pid: &ObjectId) -> Option<PrepList> {
        self.find_one("prep_list", doc! { "_id": _pid.clone() }, None)
    }

    pub fn create_prep_list(&self, _prep_list: &PrepListRequest) -> Option<ObjectId> {
        match MongoDB::doc_from(_prep_list) {
            Some(ndoc) => self.create_one::<PrepList>("prep_list", ndoc, None),
            None => None,
        }
    }

    pub fn update_prep_list(&self, _id: &ObjectId, _prep_list: &PrepListRequest) -> bool {
        let id = _id.clone();
        match MongoDB::doc_from(_prep_list) {
            Some(udoc) => {
                let doc = udoc.clone();
                self.update_one::<PrepList>("prep_list", doc! { "_id": id }, doc, None)
            }
            None => false,
        }
    }

    pub fn delete_prep_list(&self, _id: &ObjectId) -> bool {
        self.delete_one::<PrepList>("prep_list", doc! { "_id": _id.clone() }, None)
    }
}
