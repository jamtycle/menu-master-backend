use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateOptions},
    sync::{Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MongoDB {
    collections: HashMap<String, Collection<Document>>,
}

impl MongoDB {
    pub fn init() -> Self {
        let client = Client::with_uri_str(
            "mongodb://bramirez:bramirez%4006~@170.187.155.55:27017/?retryWrites=true&w=majority",
        )
        .expect("Cannot connect to database!");
        let db = client.database("menu-master");
        let mut collections: HashMap<String, Collection<Document>> = HashMap::new();

        let col_names = db
            .list_collection_names(None)
            .expect("Error getting collections!");
        
        for name in col_names.iter() {
            let col = db.collection(name);
            collections.insert(name.to_string(), col);
        }

        MongoDB { collections }
    }

    pub fn get_collection(&self, _name: &str) -> Option<&Collection<Document>> {
        match self.collections.get(_name) {
            Some(x) => Some(x),
            None => {
                println!("Collection {:?} not found!", _name);
                None
            }
        }
    }

    pub fn find<T>(
        &self,
        _collection_name: &str,
        _filter: Document,
        _options: Option<FindOptions>,
    ) -> Option<Vec<T>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection(_collection_name);
        if col.is_none() {
            return None;
        }

        match col.unwrap().find(_filter, _options) {
            Ok(data) => data
                .into_iter()
                .filter_map(Result::ok)
                .map(|x| MongoDB::doc_to::<T>(x))
                .collect(),
            Err(ex) => {
                println!("Find DB Error: {:?}", ex);
                None
            }
        }
    }

    pub fn  find_one<T>(
        &self,
        _collection_name: &str,
        _filter: Document,
        _options: Option<FindOneOptions>,
    ) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection(_collection_name);
        if col.is_none() {
            return None;
        }

        match col.unwrap().find_one(_filter, _options) {
            Ok(doc) => {
                if doc.is_none() {
                    return None;
                }

                // doc.map(|x| MongoDB::doc_to::<T>(x))?
                MongoDB::doc_to::<T>(doc.unwrap().clone())
            }
            Err(ex) => {
                println!("Find One DB Error: {:?}", ex);
                None
            }
        }
    }

    pub fn create_one<T>(
        &self,
        _collection_name: &str,
        _doc: Document,
        _options: Option<InsertOneOptions>,
    ) -> Option<ObjectId>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection(_collection_name);
        if col.is_none() {
            return None;
        }

        match col.unwrap().insert_one(_doc, _options) {
            Ok(doc) => doc.inserted_id.as_object_id(),
            Err(ex) => {
                println!("Find One DB Error: {:?}", ex);
                None
            }
        }
    }

    pub fn update_one<T>(
        &self,
        _collection_name: &str,
        _query: Document,
        _doc: Document,
        _options: impl Into<Option<UpdateOptions>>,
    ) -> bool
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection(_collection_name);
        if col.is_none() {
            return false;
        }

        let upd = doc! { "$set": _doc };
        match col.unwrap().update_one(_query, upd, _options) {
            Ok(doc) => {
                if doc.matched_count == 0 || doc.modified_count == 0 {
                    return false;
                }

                true
            }
            Err(ex) => {
                println!("Find One DB Error: {:?}", ex);
                false
            }
        }
    }

    pub fn delete_one(
        &self,
        _collection_name: &str,
        _doc: Document,
        _options: Option<DeleteOptions>,
    ) -> bool {
        let col = self.get_collection(_collection_name);
        if col.is_none() {
            return false;
        }

        match col.unwrap().delete_one(_doc, _options) {
            Ok(doc) => doc.deleted_count == 1,
            Err(ex) => {
                println!("Find One DB Error: {:?}", ex);
                false
            }
        }
    }

    pub fn doc_to<T>(_doc: Document) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        match mongodb::bson::from_document::<T>(_doc) {
            Ok(info) => Some(info),
            Err(ex) => {
                println!("Deserialize Error: {:?}", ex);
                None
            }
        }
    }

    pub fn doc_from<T>(_entity: T) -> Option<Document>
    where
        T: Serialize,
    {
        match mongodb::bson::to_document(&_entity) {
            Ok(info) => Some(info),
            Err(ex) => {
                println!("Serialization Error: {:?}", ex);
                None
            }
        }
    }
}
