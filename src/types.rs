use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserCredentials {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl UserCredentials {
    pub fn random_credentials() -> Self {
        let random_username = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
        let random_email = random_username.clone() + "@gmail.com";

        UserCredentials {
            email: random_email,
            name: random_username,
            password: "password".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TeamCredentials {
    pub name: String,
    pub password: String,
}

impl TeamCredentials {
    pub fn team_for_user(user_credentials: &UserCredentials) -> Self {
        TeamCredentials {
            name: user_credentials.name.clone() + "'s team",
            password: user_credentials.password.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessResultData {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessResult {
    pub success: Option<bool>,
    pub data: Option<GuessResultData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessBody {
    pub challenge_id: u8,
    pub submission: String,
}
