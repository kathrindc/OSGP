use crate::models::User;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseBody {
    Message(String),
    User(User),
    Users(Vec<User>),
    None,
}

#[derive(Serialize)]
pub struct Response {
    pub ok: bool,
    pub body: ResponseBody,
}
