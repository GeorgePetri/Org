use std::env;

use rocket::http::RawStr;

pub fn tenant() -> String {
    env::var("ORG_MICROSOFT_TENANT").unwrap()
}

pub fn client_id() -> String {
    env::var("ORG_MICROSOFT_CLIENT_ID").unwrap()
}

pub fn redirect_uri() -> String {
    let redirect_uri = env::var("ORG_MICROSOFT_REDIRECT_URI").unwrap();
    RawStr::percent_encode(RawStr::new(&redirect_uri)).to_string()
}

pub fn scope() -> String {
    let scope = env::var("ORG_MICROSOFT_SCOPE").unwrap();
    RawStr::percent_encode(RawStr::new(&scope)).to_string()
}
