use rocket::data::{Data, ToByteUnit};
use rocket::response::Debug;
use rocket::tokio::fs::File;
use rocket_dyn_templates::Template;
use crate::paste_id::PasteId;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", "{}")
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(6);
    let filename = format!("upload/{id}", id = id.to_string());
    let url = format!(
        "{host}/{id}",
        host = "http://localhost:8000",
        id = id.to_string()
    );
    // let file =

    paste.open(128_i32.kibibytes()).into_file(filename).await?;
    Ok(url)
}


#[get("/<id>")]
async fn get_paste(id: PasteId<'_>) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).await.ok()
}

pub fn api_routes() -> Vec<rocket::Route> {
    rocket::routes![index, upload, get_paste]
}
