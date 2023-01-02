use std::{ops::Deref, env};
use dotenv::dotenv;
use rocket::{http, request::{self, FromRequest}};
use rocket::{Request, State};
use chrono::Utc;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, DateTime, doc},
    results::{InsertOneResult, DeleteResult, UpdateResult},
    sync::{Client, Collection}
};

use crate::data::db::{User, RequestUser};

pub struct MongoConnection{
    col: Collection<User>
}

impl MongoConnection {
    pub fn init() -> Self {
        dotenv().ok();
        let url = match env::var("MONGOURL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading environment variable")
        };
        let client = Client::with_uri_str(url).unwrap();
        let db = client.database("rustapitest");
        let col: Collection<User> = db.collection("User");
        MongoConnection { col }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).expect("Error getting lists of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id };
        let user_detail = self.col.find_one(filter, None).ok().expect("Error retrieving user details");
        Ok(user_detail.unwrap())
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None).ok().expect("Unable to find user for deletion");
        Ok(user_detail)
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_user_doc = User{
            id: new_user.id,
            name: new_user.name,
            email: new_user.email,
            hashed_password: new_user.hashed_password,
            salt: new_user.salt,
            created: Utc::now(),
            updated: Utc::now()
        };
        let user = self.col.insert_one(new_user_doc, None).ok().expect("Error creating new user");
        Ok(user)
    }

    pub fn update_user(&self, id: &String, user_details: RequestUser) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": obj_id,
                "name": user_details.name,
                "email": user_details.email,
                "hashed_password": user_details.password
            }
        };
        let updated_user = self.col.update_one(filter, new_doc, None).ok().expect("Error updating user information");
        Ok(updated_user)
    }
}