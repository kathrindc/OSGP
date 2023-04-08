mod database;
mod handlers;
mod models;
mod respond;
mod schema;
mod security;

use handlers::user_handler;
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "OSGP Control API"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            index,
            user_handler::list_users_handler,
            user_handler::create_user_handler,
            user_handler::read_user_handler,
            user_handler::update_user_handler,
            user_handler::change_user_password_handler,
            user_handler::delete_user_handler,
        ],
    )
}
