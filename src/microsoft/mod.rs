use std::collections::HashMap;

use redis::Commands;
use reqwest::Url;
use rocket::response::Redirect;
use serde::Deserialize;

use crate::{OrgError, redis_data, secrets};

//todo clean this
pub use self::graph_client::{
    close_session, create_ledger, create_session, file_exists, upload_to_source,
};

pub mod data;
mod graph_client;

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

//todo impl
//todo is this the correct type, or should array be used instead?
pub async fn upload_records(session: &String, records: &Vec<Record>) -> Result<(), OrgError> {
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Token {
    token_type: String,
    scope: String,
    expires_in: i32,
    access_token: String,
    refresh_token: String,
}

//todo move since this isn't msft specific
pub struct Record {
    pub date_time: String,
    pub transaction_code: String,
    pub transaction_subcode: String,
    pub symbol: Option<String>,
    pub buy_sell: Option<String>,
    pub open_close: Option<String>,
    pub quantity: i64,
    pub price: Option<String>,
    pub fees: String,
    pub amount: String,
    pub description: String,
    pub account_reference: String,
}
