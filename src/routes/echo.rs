use rocket::get;

#[get("/echo/<echo>")]
pub fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}