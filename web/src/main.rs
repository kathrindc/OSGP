mod database;
mod handlers;
mod models;
mod respond;
mod schema;
mod security;

use dotenvy::dotenv;
use handlers::{logon_handler, user_handler};
use rocket::{get, http::Method, launch, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "OSGP Control API"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let allowed_origins = AllowedOrigins::some_exact(&[
        env::var("CORS_ORIGIN").expect("CORS_ORIGIN environment variable must be set")
    ]);
    let allowed_headers = AllowedHeaders::All;
    let allowed_methods = vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Patch,
        Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let cors = CorsOptions {
        allowed_origins,
        allowed_headers,
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration is not valid");

    rocket::build()
        .mount(
            "/api",
            routes![
                index,
                user_handler::list_users_handler,
                user_handler::create_user_handler,
                user_handler::read_user_handler,
                user_handler::update_user_handler,
                user_handler::change_user_password_handler,
                user_handler::delete_user_handler,
                logon_handler::get_logon_handler,
                logon_handler::start_logon_handler,
            ],
        )
        .attach(cors)
}
