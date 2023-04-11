use std::net::SocketAddr;

use crate::{
    models::{LogonHistory, LogonSession, User},
    respond::*,
    security::Security,
};
use rocket::{get, post, serde::json::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogonData {
    pub email: String,
    pub password: String,
}

#[get("/v1/logon")]
pub fn get_logon_handler(security: Security) -> String {
    let response = match security {
        Security::AppToken(session) => match User::load_by_id(session.user) {
            Some(user) => Response {
                ok: true,
                body: ResponseBody::User(user),
            },

            None => Response {
                ok: false,
                body: ResponseBody::Message(
                    "Your account does not or no longer exists.".to_string(),
                ),
            },
        },

        Security::ApiKey(_) => Response {
            ok: false,
            body: ResponseBody::Message("This route is not meant for direct use.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[post("/v1/logon", data = "<data>")]
pub fn start_logon_handler(remote: SocketAddr, data: Json<LogonData>) -> String {
    let response = match User::load_by_logon(data.0.email, data.0.password) {
        Some(user) => {
            let session = LogonSession::begin(user.id, remote.to_string());
            let token = Security::make_token(user, session);

            Response {
                ok: true,
                body: ResponseBody::Token(token),
            }
        }

        None => Response {
            ok: false,
            body: ResponseBody::Message("No such user with the provided credentials.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/v1/logon/history")]
pub fn get_history_handler(security: Security) -> String {
    let response = match security {
        Security::AppToken(session) => {
            let history = LogonHistory::load_by_user(session.user);

            Response {
                ok: true,
                body: ResponseBody::LogonHistory(history),
            }
        }

        Security::ApiKey(_) => Response {
            ok: false,
            body: ResponseBody::Message("This route is not meant for direct use.".to_string()),
        },
    };

    serde_json::to_string(&response).unwrap()
}
