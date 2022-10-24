use std::io::Error;

use crate::paste_id::PasteId;
use rocket::{
    config::Config,
    data::{Data, ToByteUnit},
    delete, get,
    http::Status,
    post,
    response::Debug,
    routes,
    tokio::fs::{remove_file, File},
    Route,
};
use rocket_dyn_templates::Template;

const ID_LENGTH: usize = 6;
const MAX_FILE_SIZE: i32 = 128;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", "{}")
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<(Status, String), Debug<Error>> {
    let id = PasteId::new(ID_LENGTH);

    let data_stream = paste
        .open(MAX_FILE_SIZE.kibibytes())
        .into_file(id.file_path())
        .await?;
    let status_code = if data_stream.is_complete() {
        Status::Created
    } else {
        Status::PartialContent
    };
    let base_url: String = Config::figment()
        .extract_inner("base_url")
        .unwrap_or("http://localhost:8000".to_string());
    let paste_uri = format!("{}/{}", base_url, id);
    Ok((status_code, paste_uri))
}

#[get("/<id>")]
async fn get_paste(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[delete("/<id>")]
async fn delete_paste(id: PasteId<'_>) -> Option<()> {
    remove_file(id.file_path()).await.ok()
}

pub fn api_routes() -> Vec<Route> {
    routes![index, upload, get_paste, delete_paste]
}
