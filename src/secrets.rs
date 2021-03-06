use std::env;

pub fn client_id() -> String {
    env::var("ORG_MICROSOFT_CLIENT_ID").unwrap()
}

//todo this expires in two yeas, make sure I am notified when this happens
//todo can certs help here?
pub fn client_secret() -> String {
    env::var("ORG_MICROSOFT_CLIENT_SECRET").unwrap()
}

pub fn redirect_uri() -> String {
    env::var("ORG_MICROSOFT_REDIRECT_URI").unwrap()
}

pub fn scope() -> String {
    env::var("ORG_MICROSOFT_SCOPE").unwrap()
}
