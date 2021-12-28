use std::error::Error;
use std::ffi::OsString;
use std::{env, process};
use serde::Deserialize;

//todo add everything in the proper type
#[derive(Debug, Deserialize)]
struct TastyworksRecord {
    #[serde(rename = "Fees")]
    fees: String,
    #[serde(rename = "Price")]
    price: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: TastyworksRecord = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
