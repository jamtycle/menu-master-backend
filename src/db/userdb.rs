use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    db::mongodb::MongoDB,
    model::users::{User, UserResponse},
};

use super::{mongo_tables::Tables, mongodb::MongoDBResult};

impl MongoDB {
    pub fn get_users(&self) -> Option<Vec<User>> {
        return self.find(Tables::Users.value(), doc! {}, None);
    }

    pub fn login_user(&self, _username: &String, _password: &String) -> Option<UserResponse> {
        let filter = doc! { "username": _username.clone(), "password": _password.clone() };
        let user = self.find_one::<User>(Tables::Users.value(), filter, None);
        return user.map(|u| u.into());
    }

    pub async fn register_user(&self, _user: &User) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(_user)?;
        let data = self
            .create_one::<User>(Tables::Users.value(), doc, None)
            .await?;

        Ok(data)
    }
}
