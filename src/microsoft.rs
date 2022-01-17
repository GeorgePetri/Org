use std::collections::HashMap;

use redis::{Commands, Connection};
use reqwest::Url;
use rocket::response::Redirect;
use serde::Deserialize;

use crate::secrets;

//todo add state
#[post("/login-microsoft")]
pub fn login() -> Redirect {
    let mut uri = Url::parse(&format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", secrets::tenant()))
        .unwrap();

    uri.query_pairs_mut()
        .append_pair("client_id", &secrets::client_id())
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &secrets::redirect_uri())
        .append_pair("response_mode", "query")
        .append_pair("scope", &secrets::scope());

    Redirect::to(uri.to_string())
}

#[get("/login-microsoft-callback?<code>")]
pub async fn login_callback(code: String) -> Redirect {
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

    let mut redis_connection = redis_connection();

    let _: () = redis_connection.set_ex("access_token", token.access_token, token.expires_in as usize).unwrap();

    Redirect::to("/")
}

#[get("/test")]
pub async fn test() {
    let client = reqwest::Client::new();

    let response = client.get("https://graph.microsoft.com/v1.0/me")
        .bearer_auth(access_token())
        .send()
        .await
        .unwrap();

    let text = response.text()
        .await
        .unwrap();

    println!("{}", text);
}

fn redis_connection() -> Connection {
    let redis_client = redis::Client::open("redis://127.0.0.1/")
        .unwrap();

    redis_client.get_connection()
        .unwrap()
}

fn access_token() -> String {
    let mut connection = redis_connection();

    connection.get("access_token").unwrap()
}

#[derive(Debug, Deserialize)]
struct Token {
    token_type: String,
    scope: String,
    expires_in: i32,
    access_token: String,
    refresh_token: String,
}

