#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod paste_id;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::api_routes())
        .attach(Template::fairing())
}
