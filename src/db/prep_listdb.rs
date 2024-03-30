use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::prep_list::{PrepList, PrepListRequest, PrepListResponse},
};

use super::mongo_tables::Tables;

impl MongoDB {
    pub fn get_all_prep_list(&self) -> Option<Vec<PrepListResponse>> {
        self.find::<PrepList>(Tables::PrepLists.value(), doc! {}, None)
            .map(|p| p.into_iter().map(|x| x.into()).collect())
    }

    pub fn get_prep_list(&self, _pid: &ObjectId) -> Option<PrepListResponse> {
        self.find_one::<PrepList>(
            Tables::PrepLists.value(),
            doc! { "_id": _pid.clone() },
            None,
        )
        .map(|x| x.into())
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
