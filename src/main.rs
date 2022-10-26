use rocket::{build, launch};
use rocket_dyn_templates::Template;

mod paste_id;
mod routes;
#[cfg(test)]
mod tests;

#[launch]
pub fn launch_rocket() -> _ {
    build()
        .mount("/", routes::api_routes())
        .attach(Template::fairing())
}
