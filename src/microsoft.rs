use std::env;

use rocket::http::RawStr;
use rocket::response::Redirect;

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

