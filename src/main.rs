mod  routes;
mod data;
use rocket::*;
use rocket_tut::rocket_builder;

#[launch]
fn startup_guy() -> _ {
    rocket_builder()
}

// #[launch]
// fn rocket_launcher() -> _ {
//     rocket::build()
//     .mount("/", routes![echo_fn])
//     .mount("/api", routes![
//         delete_user_rt,
//         info_user_rt,
//         update_user_tr,
//         users_list_rt,
//         new_user_rt
//     ])
// }