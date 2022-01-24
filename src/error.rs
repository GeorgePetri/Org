use std::{error, fmt, io};
use std::fmt::{Display, Formatter};

use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;

#[derive(Debug)]
pub enum OrgError {
    BadTempPath,
    MissingName,
}

impl Display for OrgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OrgError::BadTempPath => write!(f, "File could not be found at the temp location"),
            OrgError::MissingName => write!(f, "Invalid name"),
        }
    }
}

impl error::Error for OrgError {}

//todo log here
impl<'r, 'o: 'r> Responder<'r, 'o> for OrgError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        Status::InternalServerError.respond_to(request)
    }
}
