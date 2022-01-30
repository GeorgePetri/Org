use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use reqwest::StatusCode;

use crate::{hash, OrgError, redis_data};
use crate::microsoft::data::DriveItem;

//todo cache client

//todo impl properly
pub async fn create_ledger() -> Result<(), OrgError> {
    let uri = "https://graph.microsoft.com/v1.0/me/drive/root:/org/ledger.xlsx:/content";

    let client = reqwest::Client::new();
    let response = client
        .put(uri)
        .bearer_auth(redis_data::access_token())
        .header(
            "Content-Type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .header("Content-Length", 0)
        .send()
        .await?;

    let code = response.status();

    println!("code {}", response.status());
    println!("text {}", response.text().await?);

    Ok(())
}

pub async fn file_exists(path: &Path, name: &str) -> Result<bool, OrgError> {
    let client = reqwest::Client::new();

    let uri = format!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/org/source/{}",
        name
    );
    let response = client
        .get(uri)
        .bearer_auth(redis_data::access_token())
        .send()
        .await?;

    let code = response.status();

    if code == StatusCode::NOT_FOUND {
        return Ok(false);
    }

    let drive_item: DriveItem = response.json().await?;
    let drive_hash = drive_item.file.hashes.sha256_hash;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let digest = hash::sha256_digest(reader)?;
    let file_hash = hash::digest_to_upper_hex(digest);

    if drive_hash != file_hash {
        return Ok(false);
    }

    Ok(true)
}

pub async fn upload_to_source(path: &Path, name: &str) -> Result<(), OrgError> {
    let file = fs::read(&path)?;

    let uri = format!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/org/source/{}:/content",
        name
    );

    let client = reqwest::Client::new();
    let response = client
        .put(uri)
        .bearer_auth(redis_data::access_token())
        .header("Content-Type", "text/plain")
        .body(file)
        .send()
        .await?;

    let code = response.status();

    match code {
        StatusCode::OK => Ok(()),
        StatusCode::CREATED => Ok(()),
        _ => Err(OrgError::MicrosoftDrive(format!(
            "Failed uploading to drive code:{} text:{}",
            code,
            response.text().await?
        ))),
    }
}

