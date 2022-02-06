use crate::paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::response::Debug;
use rocket::tokio::fs::{remove_file, File};
use rocket_dyn_templates::Template;

const ID_LENGTH: usize = 6;
const MAX_FILE_SIZE: i32 = 128;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", "{}")
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<(Status, String), Debug<std::io::Error>> {
    let id = PasteId::new(ID_LENGTH);
    let url = format!(
        "{host}/{id}",
        host = "http://localhost:8000",
        id = id.to_string()
    );

    let data_stream = paste
        .open(MAX_FILE_SIZE.kibibytes())
        .into_file(id.file_path())
        .await?;
    let status_code = if data_stream.is_complete() {
        Status::Created
    } else {
        Status::PartialContent
    };
    Ok((status_code, url))
}

#[get("/<id>")]
async fn get_paste(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[delete("/<id>")]
async fn delete_paste(id: PasteId<'_>) -> Option<()> {
    remove_file(id.file_path()).await.ok()
}

pub fn api_routes() -> Vec<rocket::Route> {
    rocket::routes![index, upload, get_paste, delete_paste]
}
