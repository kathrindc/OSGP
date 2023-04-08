use crate::{
    models::{User, UserTrim},
    respond::{Response, ResponseBody},
};
use rocket::{delete, get, patch, post, put, serde::json::Json};

#[get("/v1/users?<page>")]
pub fn list_users_handler(page: u32) -> String {
    let users: Vec<User> = User::load_all(page);
    let response = Response {
        ok: true,
        body: ResponseBody::Users(users),
    };

    serde_json::to_string(&response).unwrap()
}

#[post("/v1/users", data = "<user_trim>")]
pub fn create_user_handler(user_trim: Json<UserTrim>) -> String {
    let user = user_trim.0.store();
    let response = Response {
        ok: true,
        body: ResponseBody::User(user),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/v1/users/<id>")]
pub fn read_user_handler(id: i32) -> String {
    let response = match User::load_by_id(id) {
        Some(user) => Response {
            ok: true,
            body: ResponseBody::User(user),
        },

        None => Response {
            ok: false,
            body: ResponseBody::Message("The user does not or no longer exists.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[put("/v1/users/<id>", data = "<user_trim>")]
pub fn update_user_handler(id: i32, user_trim: Json<UserTrim>) -> String {
    let user = User::load_by_id(id).and_then(|user| Some(user.update(user_trim.0)));
    let response = match user {
        Some(user) => Response {
            ok: true,
            body: ResponseBody::User(user),
        },

        None => Response {
            ok: false,
            body: ResponseBody::Message("The user does not or no longer exists.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[patch("/v1/users/<id>/password", data = "<password>")]
pub fn change_user_password_handler(id: i32, password: String) -> String {
    let user = User::load_by_id(id).and_then(|user| {
        user.update_password(password);
        Some(1)
    });
    let response = match user {
        Some(_) => Response {
            ok: true,
            body: ResponseBody::None,
        },

        None => Response {
            ok: false,
            body: ResponseBody::Message("The user does not or no longer exists.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[delete("/v1/users/<id>")]
pub fn delete_user_handler(id: i32) -> String {
    let response = match User::delete(id) {
        true => Response {
            ok: true,
            body: ResponseBody::None,
        },

        false => Response {
            ok: false,
            body: ResponseBody::Message("The user does not or no longer exists.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}
