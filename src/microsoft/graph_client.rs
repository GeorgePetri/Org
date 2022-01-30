use crate::{OrgError, redis_data};

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
