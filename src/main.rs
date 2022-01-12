#[macro_use]
extern crate rocket;

use std::env;

use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use rocket::http::{ContentType, RawStr};
use rocket::http::uri::{Absolute, Uri};
use rocket::response::Redirect;
use serde::Deserialize;

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

//todo add state
#[post("/login-microsoft")]
pub fn login_microsoft() -> Redirect {
    let tenant = env::var("ORG_MICROSOFT_TENANT").unwrap();
    let client_id = env::var("ORG_MICROSOFT_CLIENT_ID").unwrap();
    let redirect_uri = env::var("ORG_MICROSOFT_REDIRECT_URI").unwrap();
    let redirect_uri = RawStr::percent_encode(RawStr::new(&redirect_uri));
    let scope = env::var("ORG_MICROSOFT_SCOPE").unwrap();
    let scope = RawStr::percent_encode(RawStr::new(&scope));

    let uri = format!("\
https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
client_id={}\
&response_type=code\
&redirect_uri={}\
&response_mode=query\
&scope={}", tenant, client_id, redirect_uri, scope);

    Redirect::to(uri)
}

//todo fix copy pasted code
#[get("/login-microsoft-callback?<code>")]
pub async fn login_microsoft_callback(code: String) {
    println!("{}", code);

    let tenant = env::var("ORG_MICROSOFT_TENANT").unwrap();

    let uri = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token?", tenant);
    let response = reqwest::get(uri).await.unwrap();

    println!("{}", response.status());
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![upload, login_microsoft, login_microsoft_callback])
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
