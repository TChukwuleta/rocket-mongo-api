use rocket::http::{Status, ContentType};
mod common;


#[test]
fn echo_test() {
    let client = common::setup();
    let response = client.get("/echo/test_echo").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("test_echo".into()));
}

#[test]
fn user_lists_test() {
    let client = common::setup();
    let response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{\"status\":\"Success\",\"message\":\"List of users\"}".into()));
}

#[test]
fn new_user_test() {
    let client = common::setup();
    let response = client.post("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{\"status\":\"Success\",\"message\":\"Creation of new user\"}".into()));
}

#[test]
fn info_user_test() {
    let client = common::setup();
    let response = client.get("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{\"status\":\"Success\",\"message\":\"Info for user 1\"}".into()));
}

#[test]
fn update_user_test() {
    let client = common::setup();
    let response = client.put("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{\"status\":\"Success\",\"message\":\"Update info for user 1\"}".into()));
}

#[test]
fn delete_user_test() {
    let client = common::setup();
    let response = client.delete("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{\"status\":\"Success\",\"message\":\"Delete user 1\"}".into()));
}