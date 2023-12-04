use mongodb::bson::{doc, oid::ObjectId};

use crate::{db::mongodb::MongoDB, model::product::Product};

impl MongoDB {
    pub fn get_all_products(&self) -> Option<Vec<Product>> {
        self.find("product", doc! {}, None)
    }

    pub fn get_product(&self, _pid: &ObjectId) -> Option<Product> {
        self.find_one("product", doc! { "_id": _pid.clone() }, None)
    }

    pub fn create_product(&self, _product: &Product) -> Option<ObjectId> {
        match MongoDB::doc_from(_product) {
            Some(ndoc) => self.create_one::<Product>("product", ndoc, None),
            None => None,
        }
    }

    pub fn update_product(&self, _product: &Product) -> bool {
        let id = _product._id.clone();
        if id.is_none() {
            return false;
        }

        match MongoDB::doc_from(_product) {
            Some(udoc) => {
                let mut doc = udoc.clone();
                doc.remove("_id");
                self.update_one::<Product>("product", doc! { "_id": id }, doc, None)
            }
            None => false,
        }
    }

    pub fn delete_product(&self, _id: &ObjectId) -> bool {
        self.delete_one::<Product>("product", doc! { "_id": _id.clone() }, None)
    }
}
