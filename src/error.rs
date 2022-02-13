use std::{error, fmt, io};
use std::fmt::{Display, Formatter};
use std::io::Error;

use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;

#[derive(Debug)]
pub enum OrgError {
    BadTempPath,
    MissingName,
    Io(io::Error),
    Reqwest(reqwest::Error),
    //todo create enum for drive errors
    MicrosoftDrive404,
    MicrosoftDrive(String),
    Csv(csv::Error),
    //todo add more info
    InvalidExcel(),
}

impl Display for OrgError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            OrgError::BadTempPath => write!(f, "File could not be found at the temp location"),
            OrgError::MissingName => write!(f, "Invalid name"),
            OrgError::Io(error) => write!(f, "IO error: {}", error),
            OrgError::Reqwest(error) => write!(f, "Reqwest error: {}", error),
            OrgError::MicrosoftDrive404 => write!(f, "Microsoft Drive error 404"),
            OrgError::MicrosoftDrive(error_text) => {
                write!(f, "Microsoft Drive error: {}", error_text)
            }
            OrgError::Csv(error) => write!(f, "Csv error: {}", error),
        }
    }
}

impl error::Error for OrgError {}

impl From<io::Error> for OrgError {
    fn from(error: Error) -> Self {
        OrgError::Io(error)
    }
}

impl From<reqwest::Error> for OrgError {
    fn from(error: reqwest::Error) -> Self {
        OrgError::Reqwest(error)
    }
}

impl From<csv::Error> for OrgError {
    fn from(error: csv::Error) -> Self {
        OrgError::Csv(error)
    }
}

//todo log here
impl<'r, 'o: 'r> Responder<'r, 'o> for OrgError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        println!("Error: {}", self.to_string());
        Status::InternalServerError.respond_to(request)
    }
}
