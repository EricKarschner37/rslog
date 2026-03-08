use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index, routes::files])
        .attach(Template::fairing())
}
