use std::collections::HashMap;
use std::option::Option;

use chrono::NaiveDateTime;
use redis::Commands;
use reqwest::Url;
use rocket::response::Redirect;
use serde::Deserialize;
use serde_json::Value;

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

pub async fn get_records(session: &str) -> Result<Vec<Record>, OrgError> {
    let rows = graph_client::get_rows(session).await?;

    let mut records = Vec::new();
    for row in rows.iter() {
        match row[0] {
            Value::String(_) => {
                if !is_empty_row(row) {
                    let record = try_deserialize_record(row)?;
                    records.push(record);
                }
            }
            _ => return Err(OrgError::InvalidExcel()),
        }
    }

    Ok(records)
}

fn try_deserialize_record(row: &[Value]) -> Result<Record, OrgError> {
    fn try_match_string(value: &Value) -> Result<String, OrgError> {
        match value {
            Value::String(string) => Ok(string.clone()),
            _ => return Err(OrgError::InvalidExcel()),
        }
    }

    fn try_match_opt_string(value: &Value) -> Result<Option<String>, OrgError> {
        let string = try_match_string(value)?;

        Ok(if string.is_empty() {
            None
        } else {
            Some(string.clone())
        })
    }

    let date_time = try_match_string(&row[0])?;
    //todo remove unwrap
    let date_time = NaiveDateTime::parse_from_str(date_time.as_str(), "%d.%m.%y %I:%M %p").unwrap();
    let transaction_code = try_match_string(&row[1])?;
    let transaction_subcode = try_match_string(&row[2])?;
    let symbol = try_match_opt_string(&row[3])?;
    let buy_sell = try_match_opt_string(&row[4])?;
    let open_close = try_match_opt_string(&row[5])?;
    let quantity = match &row[6] {
        Value::Number(number) => match number.as_i64() {
            Some(value) => value,
            None => return Err(OrgError::InvalidExcel()),
        },
        _ => return Err(OrgError::InvalidExcel()),
    };
    let price = match &row[7] {
        Value::String(string) => {
            if string.is_empty() {
                None
            } else {
                Some(string.clone())
            }
        }
        Value::Number(number) => Some(number.to_string()),
        _ => return Err(OrgError::InvalidExcel()),
    };
    let fees = match &row[8] {
        //todo fix unwrap
        Value::Number(number) => number.as_f64().unwrap().to_string(),
        _ => return Err(OrgError::InvalidExcel()),
    };
    let amount = match &row[9] {
        Value::Number(number) => number.to_string(),
        _ => return Err(OrgError::InvalidExcel()),
    };
    let description = try_match_string(&row[10])?;
    let account_reference = try_match_string(&row[11])?;

    Ok(Record {
        date_time,
        transaction_code,
        transaction_subcode,
        symbol,
        buy_sell,
        open_close,
        quantity,
        price,
        fees,
        amount,
        description,
        account_reference,
    })
}

fn is_empty_row(row: &[Value]) -> bool {
    for value in row {
        match value {
            Value::String(string) => {
                if !string.is_empty() {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

//todo code looks bad
//todo try using a serializer
pub async fn upload_records(session: &str, records: Vec<Record>) -> Result<(), OrgError> {
    if records.is_empty() {
        return Ok(());
    }

    fn format_str(string: String) -> String {
        format!("\"{string}\"")
    }
    fn format_option(option: Option<String>) -> String {
        match option {
            None => "null".to_string(),
            Some(value) => format_str(value),
        }
    }

    let mut values = "{\"values\": [".to_string();
    for record in records.into_iter() {
        let mut string = "[".to_string();
        string.push_str(
            format_str(record.date_time.format("%d.%m.%y %I:%M %p").to_string()).as_str(),
        );
        string.push_str(", ");
        string.push_str(format_str(record.transaction_code).as_str());
        string.push_str(", ");
        string.push_str(format_str(record.transaction_subcode).as_str());
        string.push_str(", ");
        string.push_str(format_option(record.symbol).as_str());
        string.push_str(", ");
        string.push_str(format_option(record.buy_sell).as_str());
        string.push_str(", ");
        string.push_str(format_option(record.open_close).as_str());
        string.push_str(", ");
        string.push_str(record.quantity.to_string().as_str());
        string.push_str(", ");
        string.push_str(format_option(record.price).as_str());
        string.push_str(", ");
        string.push_str(format_str(record.fees).as_str());
        string.push_str(", ");
        string.push_str(format_str(record.amount).as_str());
        string.push_str(", ");
        string.push_str(format_str(record.description).as_str());
        string.push_str(", ");
        string.push_str(format_str(record.account_reference).as_str());
        string.push_str("]");

        values.push_str(&string);
        values.push_str(", ");
    }
    values.truncate(values.len() - 2);
    values.push_str("]}");

    graph_client::create_rows(session, values).await?;

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
//todo should these fields be &str?
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Record {
    pub date_time: NaiveDateTime,
    pub transaction_code: String,
    pub transaction_subcode: String,
    pub symbol: Option<String>,
    pub buy_sell: Option<String>,
    pub open_close: Option<String>,
    pub quantity: i64,
    pub price: Option<String>,
    //todo fix fees type
    pub fees: String,
    pub amount: String,
    pub description: String,
    pub account_reference: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_row_returns_true_when_empty_input() {
        let x: Vec<Value> = [0..10]
            .iter()
            .map(|_| Value::String("".to_string()))
            .collect();

        let result = is_empty_row(&x);

        assert!(result)
    }

    #[test]
    fn is_empty_row_returns_false_when_not_empty_input() {
        let x: Vec<Value> = [0..10]
            .iter()
            .map(|_| Value::String("a".to_string()))
            .collect();

        let result = is_empty_row(&x);

        assert!(!result)
    }
}
