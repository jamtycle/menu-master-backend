use mongodb::bson::oid::ObjectId;

use super::response::APIError;

pub fn parse_object_id(id: &str) -> Result<ObjectId, APIError> {
    ObjectId::parse_str(id).map_err(|ex| APIError::BadRequest(ex.to_string()))
}