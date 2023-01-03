use chrono::Utc;
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{get, post, put, delete, serde::json::Json, http::Status, State};
use crate::data::{db::{User, RequestUser}, mongo_connection::MongoConnection};

// pub struct ApiResponse {
//     status: Status,
//     user: Option<Json<Vec<User>>>,
//     message: String
// }

// impl ApiResponse {
//     pub fn ok(message: &str) -> Self {
//         ApiResponse { status: Status::Ok, message: message.to_owned(), user: None }
//     }

//     pub fn ok_with_data(message: &str, data: Vec<User>) -> Self {
//         ApiResponse { status: Status::Ok, user: Some(Json(data)), message: message.to_owned() }
//     }

//     pub fn error(message: String, status: Status) -> Self {
//         ApiResponse { status, message, user: None }
//     }

//     pub fn internal_err() -> Self {
//         ApiResponse { status: Status::InternalServerError, message: String::from("Internal server error"), user: None }
//     }
// }

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
            println!("Taniii");
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

#[post("/createuser", data="<new_user>")]
pub fn new_user_rt(db: &State<MongoConnection>, new_user: Json<RequestUser>) -> Result<Json<InsertOneResult>, Status> {
    let test = RequestUser{
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned()
    };
    let user_detail = User::from_insertable(test);
    let data = User {
        id: user_detail.id,
        name: user_detail.name,
        email: user_detail.email,
        hashed_password: user_detail.hashed_password,
        salt: user_detail.salt,
        created: user_detail.created,
        updated: user_detail.updated
    };
    let user = db.create_user(data);
    match user {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/users/<id>")]
pub fn info_user_rt(db:&State<MongoConnection>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::InternalServerError)
    }
    let user = db.get_user(&id);
    match user {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/updateuser/<id>", data = "<new_user>")]
pub fn update_user_tr(db: &State<MongoConnection>, id: String, new_user: Json<RequestUser>) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    
    let existing_user = db.get_user(&id).unwrap();
    //let user_data = user_info?.update_user(&new_user.name, &new_user.email);
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        hashed_password: existing_user.hashed_password,
        salt: existing_user.salt,
        created: existing_user.created,
        updated: Utc::now()
    };
    println!("{:?}", &data);
    let updated_user = db.update_user(&id, data);
    match updated_user {
        Ok(c) => {
            if c.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError)
                };
            }
            else {
                return Err(Status::NotFound);
            }
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/users/<id>")]
pub fn delete_user_rt(db: &State<MongoConnection>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::InternalServerError)
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted"));
            }
            else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError)
    }
}