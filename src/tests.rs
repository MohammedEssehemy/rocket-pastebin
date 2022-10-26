use super::{
    launch_rocket,
    paste_id::PasteId,
    routes::{
        rocket_uri_macro_delete_paste, rocket_uri_macro_get_paste, rocket_uri_macro_index,
        rocket_uri_macro_upload,
    },
};
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
    request::FromParam,
    uri,
};
use rocket_dyn_templates::{context, Template};

fn extract_id(from: &str) -> Option<String> {
    from.rfind('/')
        .map(|i| &from[(i + 1)..])
        .map(|s| s.trim_end().to_string())
}

#[test]
fn check_index() {
    let client = Client::tracked(launch_rocket()).unwrap();

    // Ensure the index returns what we expect.
    let response = client.get(uri!(index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(
        response.into_string(),
        Some(
            Template::show(
                client.rocket(),
                "index",
                context! {
                    base_url: "http://localhost:8000"
                },
            )
            .unwrap()
        )
    );
}

fn upload_paste(client: &Client, body: &str) -> String {
    let response = client.post(uri!(upload)).body(body).dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    extract_id(&response.into_string().unwrap()).unwrap()
}

fn download_paste(client: &Client, id: &str) -> Option<String> {
    let id = PasteId::from_param(id).expect("valid ID");
    let response = client.get(uri!(get_paste(id))).dispatch();
    if response.status().class().is_success() {
        Some(response.into_string().unwrap())
    } else {
        None
    }
}

fn delete_paste_local(client: &Client, id: &str) {
    let id = PasteId::from_param(id).expect("valid ID");
    let response = client.delete(uri!(delete_paste(id))).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn pasting() {
    let client = Client::tracked(launch_rocket()).unwrap();

    // Do a trivial upload, just to make sure it works.
    let body_1 = "Hello, world!";
    let id_1 = upload_paste(&client, body_1);
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);

    // Make sure we can keep getting that paste.
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);

    // Upload some unicode.
    let body_2 = "こんにちは";
    let id_2 = upload_paste(&client, body_2);
    assert_eq!(download_paste(&client, &id_2).unwrap(), body_2);

    // Make sure we can get both pastes.
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);
    assert_eq!(download_paste(&client, &id_2).unwrap(), body_2);
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);
    assert_eq!(download_paste(&client, &id_2).unwrap(), body_2);

    // Now a longer upload.
    let body_3 = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
        aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit
        in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui
        officia deserunt mollit anim id est laborum.";

    let id_3 = upload_paste(&client, body_3);
    assert_eq!(download_paste(&client, &id_3).unwrap(), body_3);
    assert_eq!(download_paste(&client, &id_1).unwrap(), body_1);
    assert_eq!(download_paste(&client, &id_2).unwrap(), body_2);

    // Delete everything we uploaded.
    delete_paste_local(&client, &id_1);
    assert!(download_paste(&client, &id_1).is_none());

    delete_paste_local(&client, &id_2);
    assert!(download_paste(&client, &id_2).is_none());

    delete_paste_local(&client, &id_3);
    assert!(download_paste(&client, &id_3).is_none());
}
