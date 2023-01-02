use rocket_tut::rocket_builder;
use rocket::local::blocking::Client;

pub fn setup() -> Client {
    Client::new(rocket_builder()).expect("Valid Rocket instance")
}