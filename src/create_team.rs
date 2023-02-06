use serde::{Deserialize, Serialize};

use crate::{get_nonce_for_page, TeamCredentials};

#[derive(Debug, Serialize, Deserialize)]
struct CreateTeamData {
    name: String,
    password: String,
    nonce: String,
}

pub fn create_team(
    client: &reqwest::blocking::Client,
    team_credentials: &TeamCredentials,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let teams_nonce = get_nonce_for_page(client, "/teams/new").unwrap();

    let team_data = CreateTeamData {
        nonce: teams_nonce,
        name: team_credentials.name.clone(),
        password: team_credentials.password.clone(),
    };

    let response = client
        .post("https://ctf.acm.umn.edu/teams/new")
        .form(&team_data)
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        println!("{:#?}", response);
        panic!(
            "Cloud not create team {}, you might need to check the csrf nonce or the cookies",
            team_credentials.name
        );
    }

    Ok(response)
}
