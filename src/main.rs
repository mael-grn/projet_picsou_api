mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;
mod auth;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::response::content;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: u16,
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: status.code })
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Picsou Project!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![default_catcher])
        .mount("/", routes![index])
        .mount("/auth", routes![routes::auth::login, routes::auth::register, routes::auth::validate])
}