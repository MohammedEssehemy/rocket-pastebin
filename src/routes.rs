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
use rocket_dyn_templates::{context, Template};

const ID_LENGTH: usize = 6;
const MAX_FILE_SIZE: i32 = 128;

fn get_base_url() -> String {
    Config::figment()
        .extract_inner("base_url")
        .unwrap_or("http://localhost:8000".to_string())
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<File> {
    File::open("./static/favicon.ico").await.ok()
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            base_url: get_base_url()
        },
    )
}

#[post("/", data = "<paste>")]
pub async fn upload(paste: Data<'_>) -> Result<(Status, String), Debug<Error>> {
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
    let paste_uri = format!("{}/{}", get_base_url(), id);
    Ok((status_code, paste_uri))
}

#[get("/<id>")]
pub async fn get_paste(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[delete("/<id>")]
pub async fn delete_paste(id: PasteId<'_>) -> Option<()> {
    remove_file(id.file_path()).await.ok()
}

pub fn api_routes() -> Vec<Route> {
    routes![index, favicon, upload, get_paste, delete_paste]
}
