#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use rocket::http::ContentType;
use serde::Deserialize;


//todo can i remove this type?
#[derive(FromForm)]
pub struct FileUploadForm<'f> {
    #[field(validate = ext(ContentType::CSV))]
    file: TempFile<'f>,
}

//todo nice error handling
//todo does this need to be async?
#[post("/upload", data = "<form>")]
pub async fn upload(form: Form<FileUploadForm<'_>>) {
    match form.file.path() {
        Some(path) => {
            let mut reader = csv::Reader::from_path(path).unwrap();

            for result in reader.deserialize() {
                let record: TastyworksRecord = result.unwrap();
                println!("{:?}", record);
            }
        }
        None => ()
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![upload])
        .mount("/", FileServer::from("static/"))
}

//todo add everything in the proper type
#[derive(Debug, Deserialize)]
struct TastyworksRecord {
    #[serde(rename = "Fees")]
    fees: String,
    #[serde(rename = "Price")]
    price: String,
}
