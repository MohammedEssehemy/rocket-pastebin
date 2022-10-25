use rocket::{build, launch};
use rocket_dyn_templates::Template;

mod paste_id;
mod routes;

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes::api_routes())
        .attach(Template::fairing())
}
