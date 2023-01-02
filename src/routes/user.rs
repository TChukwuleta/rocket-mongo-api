use mongodb::bson::extjson::de::Error;
use rocket::{get, post, put, delete, serde::json::Json, http::Status, response::{Responder, Response}, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data::{db::User, mongo_connection::MongoConnection};

pub struct ApiResponse {
    status: Status,
    user: Option<Json<Vec<User>>>,
    message: String
}

impl ApiResponse {
    pub fn ok(message: &str) -> Self {
        ApiResponse { status: Status::Ok, message: message.to_owned(), user: None }
    }

    pub fn ok_with_data(message: &str, data: Vec<User>) -> Self {
        ApiResponse { status: Status::Ok, user: Some(Json(data)), message: message.to_owned() }
    }

    pub fn error(message: String, status: Status) -> Self {
        ApiResponse { status, message, user: None }
    }

    pub fn internal_err() -> Self {
        ApiResponse { status: Status::InternalServerError, message: String::from("Internal server error"), user: None }
    }
}

#[get("/users")]
pub fn users_list_rt(db: &State<MongoConnection>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    // let mut message = String::new();
    // if users.is_ok() {
    //     message = String::from("Users information retrieval was successful");
    //     ApiResponse::ok_with_data(&message, users.unwrap())
    // }
    // else {
    //     message = String::from("Error retrieving users detals");
    //     ApiResponse::error(message, Status::NotFound)
    // }
    match users {
        Ok(users) =>{ 
            //Ok(ApiResponse::ok_with_data("Data retrieval was successful", users))
            Ok(Json(users))
        },
        Err(_) => {
            //let error_message = format!("Users retrieval was not successful. {}", e);
            //Ok(ApiResponse::error(error_message, Status::NotFound))
            Err(Status::InternalServerError)
        }
    }
}

#[post("/users")]
pub fn new_user_rt() -> Json<ResultResponse> {
    Json(ResultResponse::ok("Creation of new user"))
}

#[get("/users/<id>")]
pub fn info_user_rt(id: String) -> Json<ApiResponse> {

   Json(ApiResponse::ok(&* format!("Info for user {}", id)))
}

#[put("/users/<id>")]
pub fn update_user_tr(id: String) -> Json<ResultResponse> {
    Json(ResultResponse::ok(&* format!("Update info for user {}", id)))
}

#[delete("/users/<id>")]
pub fn delete_user_rt(id: String) -> Json<ResultResponse> {
    Json(ResultResponse::ok(&* format!("Delete user {}", id)))
}