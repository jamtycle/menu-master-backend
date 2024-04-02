use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error as MongoDBError,
    options::{DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateOptions},
    sync::{Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MongoDB {
    collections: HashMap<String, Collection<Document>>,
}

pub type MongoDBResult<T> = Result<T, MongoDBError>;

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

    pub fn get_collection_res(&self, _name: &str) -> Result<&Collection<Document>, MongoDBError> {
        self.collections
            .get(_name)
            .ok_or(MongoDBError::custom("Collection does not exists!"))
    }

    pub async fn find_res<T>(
        &self,
        _collection_name: &str,
        _filter: Document,
        _options: Option<FindOptions>,
    ) -> Result<Vec<T>, MongoDBError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection_res(_collection_name)?;

        let data = col.find(_filter, _options)?;
        return data
            .map(|x| x.and_then(|doc| mongodb::bson::from_document::<T>(doc).map_err(Into::into)))
            .collect::<Result<Vec<T>, mongodb::error::Error>>();
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

    pub fn find_one<T>(
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

    pub async fn create_one<T>(
        &self,
        _collection_name: &str,
        _doc: Document,
        _options: impl Into<Option<InsertOneOptions>>,
    ) -> Result<ObjectId, MongoDBError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection_res(_collection_name)?;

        col.insert_one(_doc, _options)
            .map(|x| x.inserted_id.as_object_id().unwrap())
    }

    pub async fn update_one<T>(
        &self,
        _collection_name: &str,
        _query: Document,
        _doc: Document,
        _options: impl Into<Option<UpdateOptions>>,
    ) -> Result<bool, MongoDBError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let col = self.get_collection_res(_collection_name)?;

        if _doc.keys().any(|x| x.contains("$")) {
            return col
                .update_one(_query, _doc, _options)
                .map(|x| x.matched_count == 1 && x.modified_count == 1);
        }

        let upd = doc! { "$set": _doc };
        return col
            .update_one(_query, upd, _options)
            .map(|x| x.matched_count == 1 && x.modified_count == 1);
    }

    pub async fn delete_one(
        &self,
        _collection_name: &str,
        _doc: Document,
        _options: impl Into<Option<DeleteOptions>>,
    ) -> Result<bool, MongoDBError> {
        let col = self.get_collection_res(_collection_name)?;

        col.delete_one(_doc, _options).map(|x| x.deleted_count == 1)
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

    pub fn doc_from<T>(_entity: T) -> Result<Document, MongoDBError>
    where
        T: Serialize,
    {
        mongodb::bson::to_document(&_entity).map_err(|x| x.into())
    }
}
