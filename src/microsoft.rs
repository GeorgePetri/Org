use rocket::response::Redirect;

use crate::secrets;

//todo add state
#[post("/login-microsoft")]
pub fn login_microsoft() -> Redirect {
    let uri = format!("\
https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
client_id={}\
&response_type=code\
&redirect_uri={}\
&response_mode=query\
&scope={}", secrets::tenant(), secrets::client_id(), secrets::redirect_uri(), secrets::scope());

    Redirect::to(uri)
}

#[get("/login-microsoft-callback?<code>")]
pub async fn login_microsoft_callback(code: String) {
    println!("{}", code);

    let uri = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token?", secrets::tenant());
    let response = reqwest::get(uri).await.unwrap();

    println!("{}", response.status());
}

