use crate::{get_nonce_for_page, GuessBody, GuessResult};

pub fn attempt(
    client: &reqwest::blocking::Client,
    submission: usize,
) -> Result<GuessResult, Box<dyn std::error::Error>> {
    let challenges_nonce = get_nonce_for_page(client, "/challenges").unwrap();

    let guess_data = GuessBody {
        challenge_id: 7,
        submission: submission.to_string(),
    };

    let response = client
        .post("https://ctf.acm.umn.edu/api/v1/challenges/attempt")
        .header("csrf-token", challenges_nonce)
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&guess_data)
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        println!("{:#?}", response);
        panic!("Cloud not submit, you might need to check the csrf nonce or the cookies");
    }

    let guess_response = response.json::<GuessResult>().unwrap();
    Ok(guess_response)
}
