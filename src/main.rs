#[macro_use]
extern crate rocket;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use chrono::NaiveDateTime;
use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::ContentType;
use rocket::response::Redirect;
use serde::Deserialize;
use serde_json::Number;

use crate::error::OrgError;
use crate::microsoft::Record;

mod error;
mod hash;
mod microsoft;
mod redis_data;
mod secrets;

#[derive(FromForm)]
pub struct FileUploadForm<'f> {
    #[field(validate = ext(ContentType::CSV))]
    file: TempFile<'f>,
}

//use a persistent session for manipulating the excel
//create the excel there, if not existing
//create a worksheet if not existing
//merge uploaded data in excel
//save and close session
#[post("/upload", data = "<form>")]
pub async fn upload(form: Form<FileUploadForm<'_>>) -> Result<Redirect, OrgError> {
    let path = form.file.path().ok_or(OrgError::BadTempPath)?;
    let name_without_extension = form.file.name().ok_or(OrgError::MissingName)?;
    let extension = form
        .file
        .content_type()
        .ok_or(OrgError::BadTempPath)?
        .extension()
        .ok_or(OrgError::BadTempPath)?
        .as_str();
    let name = format!("{}.{}", name_without_extension, extension);

    let already_exists = microsoft::file_exists(path, &name).await?;

    //todo uncomment this
    // if !already_exists {
    microsoft::upload_to_source(path, &name).await?;

    let session = microsoft::create_session().await;

    let session = match session {
        Ok(result) => result,
        Err(error) => match error {
            OrgError::MicrosoftDrive404 => {
                microsoft::create_ledger().await?;
                //todo currently there is period of time between when the ledger is uploaded and a session can be crated this should be fixed in the future with an async flow
                microsoft::create_session().await?
            }
            _ => return Err(error),
        },
    };

    let old_records = microsoft::get_records(&session).await?;
    let new_records = build_records(path)?;
    let records_to_upload = diff_records(old_records, new_records);
    microsoft::upload_records(&session, records_to_upload).await?;
    microsoft::close_session(&session).await?;
    // }

    Ok(Redirect::to(uri!("/")))
}

fn build_records(path: &Path) -> Result<Vec<Record>, OrgError> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let result: TastyworksRecord = result?;
        records.push(result.to_record());
    }

    Ok(records)
}

fn diff_records<'a>(old: Vec<Record>, new: Vec<Record>) -> Vec<Record> {
    let old: HashSet<Record> = HashSet::from_iter(old);

    let mut result = Vec::new();

    dbg!(&old);

    for record in new {
        if !old.contains(&record) {
            println!("not contains {:?}", record);
            result.push(record);
        } else {
            println!("contains {:?}", record);
        }
    }

    result
}

//todo add tests
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![index, upload, microsoft::login, microsoft::login_callback],
    )
}

//todo add logout feature
#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = if redis_data::has_access_token() {
        "static/index.html"
    } else {
        "static/login.html"
    };
    NamedFile::open(Path::new(path)).await.ok()
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
    fees: Number,
    #[serde(rename = "Amount")]
    amount: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Account Reference")]
    account_reference: String,
}

//todo this clones don't look good, should they change?
//todo timezone info
//todo remove unwrap, use error
impl TastyworksRecord {
    fn to_record(self) -> Record {
        let date_time =
            NaiveDateTime::parse_from_str(self.date_time.as_str(), "%m/%d/%Y %I:%M %p").unwrap();
        Record {
            date_time,
            transaction_code: self.transaction_code,
            transaction_subcode: self.transaction_subcode,
            symbol: self.symbol,
            buy_sell: self.buy_sell,
            open_close: self.open_close,
            quantity: self.quantity,
            price: self.price,
            //todo fix unwrap
            fees: self.fees.as_f64().unwrap().to_string(),
            amount: self.amount,
            description: self.description,
            account_reference: self.account_reference,
        }
    }
}
