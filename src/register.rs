use serde::{Deserialize, Serialize};

use crate::{get_nonce_for_page, UserCredentials};

#[derive(Debug, Serialize, Deserialize)]
struct RegisterData {
    name: String,
    email: String,
    password: String,
    nonce: String,
}

pub fn register(
    client: &reqwest::blocking::Client,
    user_credentials: &UserCredentials,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let register_nonce = get_nonce_for_page(client, "/register").unwrap();

    let register_data = RegisterData {
        nonce: register_nonce,
        name: user_credentials.name.clone(),
        email: user_credentials.email.clone(),
        password: user_credentials.password.clone(),
    };

    let response = client
        .post("https://ctf.acm.umn.edu/register")
        .form(&register_data)
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        println!("{:#?}", response);
        panic!(
            "Cloud not create user {}, you might need to check the csrf nonce or the cookies",
            user_credentials.name
        );
    }

    Ok(response)
}
