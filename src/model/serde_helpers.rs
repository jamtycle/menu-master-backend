use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::Serializer;

pub fn serialize_option_object_id_as_hex_string<S: Serializer>(
    oid: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match oid {
        Some(oid) => serialize_object_id_as_hex_string(oid, serializer),
        None => serializer.serialize_none(),
    }
}
