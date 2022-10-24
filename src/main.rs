use rocket::{build, fairing::AdHoc, launch, Config};
use rocket_dyn_templates::Template;

mod paste_id;
mod routes;

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes::api_routes())
        .attach(AdHoc::config::<Config>())
        .attach(Template::fairing())
}
