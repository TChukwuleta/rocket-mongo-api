use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use argon2;
use std::iter::Iterator;

// User request model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  RequestUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePassword {
    pub new_password: String,
    pub id: ObjectId
}

// User response model
pub struct  ResponseUser {
    pub id: ObjectId,
    pub name: String,
    pub email: String
}

impl ResponseUser {
    pub fn from_user(user: &User) -> Self {
        ResponseUser { id: user.id.unwrap(), name: format!("{}", user.name), email: format!("{}", user.email) }
    }
}

// Change password model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPassword{
    pub password: String,
    pub new_password: Option<String>
}

// User model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let salt: String = thread_rng().sample_iter(Alphanumeric)
        .take(20).map(char::from).collect();
        let hashed_password = User::hash_password(&password, &salt);
        User { id: None, name, email, hashed_password, salt, created: Utc::now(), updated: Utc::now() }
    }

    pub fn from_insertable(insertable: RequestUser) -> Self {
        User::new(insertable.name, insertable.email, insertable.password)
    }

    pub fn match_password(&self, password: &String) -> bool {
        argon2::verify_encoded(&self.hashed_password, password.as_bytes()).unwrap()
    }

    pub fn update_user(&mut self, name: &String, email: &String) {
        self.name = name.to_string();
        self.email = email.to_string();
        self.updated = Utc::now();
    }

    pub fn update_password(&mut self, password: &String) {
        self.hashed_password = User::hash_password(password, &self.salt);
        self.updated = Utc::now();
    }


    pub fn hash_password(password: &String, salt: &String) -> String {
        let config = argon2::Config::default();
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
    }
}