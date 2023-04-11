use crate::models::{LogonHistory, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseBody {
    Message(String),
    User(User),
    Users(Vec<User>),
    Token(String),
    LogonHistory(Vec<LogonHistory>),
    None,
}

#[derive(Serialize)]
pub struct Response {
    pub ok: bool,
    pub body: ResponseBody,
}
