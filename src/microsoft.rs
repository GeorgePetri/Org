use std::collections::HashMap;

use reqwest::Client;
use rocket::http::RawStr;
use rocket::response::Redirect;
use serde::Deserialize;

use crate::secrets;

//todo add state
//todo is there a helper to create the uri easier?
#[post("/login-microsoft")]
pub fn login_microsoft() -> Redirect {
    let redirect_uri_encoded = RawStr::percent_encode(RawStr::new(&secrets::redirect_uri())).to_string();
    let scope_encoded = RawStr::percent_encode(RawStr::new(&secrets::scope())).to_string();
    let uri = format!("\
https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
client_id={}\
&response_type=code\
&redirect_uri={}\
&response_mode=query\
&scope={}", secrets::tenant(), secrets::client_id(), redirect_uri_encoded, scope_encoded);

    Redirect::to(uri)
}

#[get("/login-microsoft-callback?<code>")]
pub async fn login_microsoft_callback(code: String) -> Redirect {
    let mut params = HashMap::new();
    params.insert("client_id", secrets::client_id());
    params.insert("scope", secrets::scope());
    params.insert("code", code);
    params.insert("redirect_uri", secrets::redirect_uri());
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("client_secret", secrets::client_secret());

    let uri = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", secrets::tenant());

    //todo reuse client
    let client = reqwest::Client::new();
    let response = client.post(uri)
        .form(&params)
        .send()
        .await
        .unwrap();

    let token: Token = response
        .json()
        .await
        .unwrap();

    println!("{:?}", token);

    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
struct Token {
    token_type: String,
    scope: String,
    expires_in: i32,
    access_token: String,
    refresh_token: String,
}

