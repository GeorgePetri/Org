use std::collections::HashMap;
use std::fs;
use std::path::Path;

use redis::Commands;
use reqwest::{StatusCode, Url};
use rocket::response::Redirect;
use serde::Deserialize;

use crate::{OrgError, redis_data, secrets};

//todo add state
#[post("/login-microsoft")]
pub fn login() -> Redirect {
    let mut uri =
        Url::parse("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize").unwrap();

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

    let uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

    //todo reuse client
    let client = reqwest::Client::new();
    let response = client.post(uri).form(&params).send().await.unwrap();

    let token: Token = response.json().await.unwrap();

    let mut redis_connection = redis_data::redis_connection();

    let _: () = redis_connection
        .set_ex(
            "access_token",
            token.access_token,
            token.expires_in as usize,
        )
        .unwrap();

    Redirect::to("/")
}

#[get("/test")]
pub async fn test() {
    let client = reqwest::Client::new();

    let response = client
        .get("https://graph.microsoft.com/v1.0/me/drive/root/children")
        .bearer_auth(redis_data::access_token())
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();

    println!("{}", text);
}

//todo fix bad error handling in this file
//todo do sha1
pub async fn file_exists(name: String, sha1: String) -> bool {
    let client = reqwest::Client::new();
    let response = client
        .get("https://graph.microsoft.com/v1.0/me/drive/root:/org/source")
        .bearer_auth(redis_data::access_token())
        .send()
        .await
        .unwrap();

    let code = response.status();

    if code == StatusCode::NOT_FOUND {
        return false;
    }

    true
}

pub async fn upload_to_source(path: &Path, name: &str) -> Result<(), OrgError> {
    try_upload_to_source(path).await;

    Ok(())
}

async fn try_upload_to_source(path: &Path) {
    let file = fs::read(&path).unwrap();
    let file_path = fs::canonicalize(&path).unwrap();

    let client = reqwest::Client::new();

    let uri = format!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/org/source/{}:/content",
        file_path.file_name().unwrap().to_str().unwrap()
    );

    let uri = format!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/org/source/{}:/content",
        "test.txt"
    );

    println!("{}", uri);

    let response = client
        .put(uri)
        .header("Content-Type", "text/plain")
        .body(file)
        .bearer_auth(redis_data::access_token())
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();

    println!("{}", text);
}

async fn ensure_source_dir_exists() {
    let client = reqwest::Client::new();
    // let response = client.get("https://graph.microsoft.com/v1.0/me/drive/root:/org/source")
    //     .bearer_auth(redis_data::access_token())
    //     .send()
    //     .await
    //     .unwrap();
}

#[derive(Debug, Deserialize)]
struct Token {
    token_type: String,
    scope: String,
    expires_in: i32,
    access_token: String,
    refresh_token: String,
}
