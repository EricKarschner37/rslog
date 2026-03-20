use std::env;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

use crate::model::{DB, run_db_migrations};

#[macro_use] extern crate rocket;

mod routes;
mod model;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let offline_mode = env::var("SQLX_OFFLINE")
        .ok()
        .and_then(|v| { print!("v: {}", v); Some(v == "true") })
        .unwrap_or(false);

    let mut r = rocket::build()
        .attach(Template::fairing());

    if !offline_mode {
        r = r.attach(DB::init())
            .attach(AdHoc::try_on_ignite("SQLx migrations", run_db_migrations));
    } else {
        print!("Starting in offline mode, skipping db connection...");
    }
    r
        .mount("/", routes![routes::index, routes::files, routes::details, routes::create_get, routes::create_post, routes::feed])
}
