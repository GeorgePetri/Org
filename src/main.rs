#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use rocket::http::ContentType;
use rocket::response::Redirect;
use serde::Deserialize;

mod microsoft;

#[derive(FromForm)]
pub struct FileUploadForm<'f> {
    #[field(validate = ext(ContentType::CSV))]
    file: TempFile<'f>,
}

//todo nice error handling
//todo does this need to be async?
//todo add tests
//todo redis cache?
#[post("/upload", data = "<form>")]
pub async fn upload(form: Form<FileUploadForm<'_>>) -> Redirect {
    match form.file.path() {
        Some(path) => {
            let mut reader = csv::Reader::from_path(path).unwrap();

            for result in reader.deserialize() {
                let record: TastyworksRecord = result.unwrap();
                println!("{:?}", record);
            }
        }
        None => ()
    };

    Redirect::to(uri!("/"))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![upload, microsoft::login_microsoft, microsoft::login_microsoft_callback])
        .mount("/", FileServer::from("static/"))
}

#[derive(Debug, Deserialize)]
struct TastyworksRecord {
    #[serde(rename = "Date/Time")]
    date_time: String,
    #[serde(rename = "Transaction Code")]
    transaction_code: String,
    #[serde(rename = "Transaction Subcode")]
    transaction_subcode: String,
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Buy/Sell")]
    buy_sell: Option<String>,
    #[serde(rename = "Open/Close")]
    open_close: Option<String>,
    #[serde(rename = "Quantity")]
    quantity: i64,
    #[serde(rename = "Expiration Date")]
    expiration_date: Option<String>,
    #[serde(rename = "Strike")]
    strike: Option<String>,
    #[serde(rename = "Call/Put")]
    call_put: Option<String>,
    #[serde(rename = "Price")]
    price: Option<String>,
    #[serde(rename = "Fees")]
    fees: String,
    #[serde(rename = "Amount")]
    amount: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Account Reference")]
    account_reference: String,
}
