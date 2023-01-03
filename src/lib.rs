use rocket::*;
use routes::{echo::echo_fn, user::{new_user_rt, info_user_rt, delete_user_rt, users_list_rt, update_user_tr}};
mod routes;
pub mod data;

use data::mongo_connection::MongoConnection;


#[launch]
pub fn rocket_builder() -> _ {
    rocket::build().
    mount("/", routes![echo_fn])
    .mount("/api", routes![
        new_user_rt,
        info_user_rt,
        delete_user_rt,
        users_list_rt,
        update_user_tr
    ])
    .manage(MongoConnection::init())
}