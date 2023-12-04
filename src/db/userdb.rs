use mongodb::bson::{doc, oid::ObjectId};

use crate::{db::mongodb::MongoDB, model::user::User};

impl MongoDB {
    pub fn get_users(&self) -> Option<Vec<User>> {
        self.find("user", doc! {}, None)
    }

    pub fn login_user(&self, _username: &String, _password: &String) -> Option<User> {
        let filter = doc! { "username": _username.clone(), "password": _password.clone() };
        self.find_one("user", filter, None)
    }

    pub fn register_user(&self, _user: &User) -> Option<ObjectId> {
        match MongoDB::doc_from(_user) {
            Some(ndoc) => self.create_one::<User>("user", ndoc, None),
            None => None,
        }
    }
}
