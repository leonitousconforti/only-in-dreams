use serde::{Deserialize, Serialize};

use crate::{get_nonce_for_page, UserCredentials};

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    name: String,
    password: String,
    nonce: String,
}

pub fn login(
    client: &reqwest::blocking::Client,
    user_credentials: &UserCredentials,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let login_nonce = get_nonce_for_page(client, "/login").unwrap();

    let login_data = LoginData {
        nonce: login_nonce,
        name: user_credentials.name.clone(),
        password: user_credentials.password.clone(),
    };

    let response = client
        .post("https://ctf.acm.umn.edu/login")
        .form(&login_data)
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        println!("{:#?}", response);
        panic!(
            "Cloud not login user {}, you might need to check the csrf nonce or the cookies",
            user_credentials.name
        );
    }

    Ok(response)
}
