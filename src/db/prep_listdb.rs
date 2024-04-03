use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::prep_list::{PrepList, PrepListRequest, PrepListResponse},
};

use super::{mongo_tables::Tables, mongodb::MongoDBResult};

impl MongoDB {
    pub async fn get_all_prep_list(&self) -> MongoDBResult<Vec<PrepListResponse>> {
        let data = self
            .find_res::<PrepList>(Tables::PrepLists.value(), doc! {}, None)
            .await?;

        Ok(data.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_prep_list(&self, _pid: &ObjectId) -> MongoDBResult<PrepListResponse> {
        let data = self
            .find_one_res::<PrepList>(
                Tables::PrepLists.value(),
                doc! { "_id": _pid.clone() },
                None,
            )
            .await?;

        Ok(data.into())
    }

    pub async fn create_prep_list(&self, _prep_list: &PrepListRequest) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(_prep_list)?;
        let data = self
            .create_one::<PrepList>(Tables::PrepLists.value(), doc, None)
            .await?;

        Ok(data)
    }

    pub async fn update_prep_list(
        &self,
        _id: &ObjectId,
        _prep_list: &PrepListRequest,
    ) -> MongoDBResult<bool> {
        let id = _id.clone();
        let doc = MongoDB::doc_from(_prep_list)?;
        let data = self
            .update_one::<PrepList>(Tables::PrepLists.value(), doc! { "_id": id }, doc, None)
            .await?;

        Ok(data)
    }

    pub async fn delete_prep_list(&self, _id: &ObjectId) -> MongoDBResult<bool> {
        let data = self
            .delete_one(Tables::PrepLists.value(), doc! { "_id": _id.clone() }, None)
            .await?;

        Ok(data)
    }
}
