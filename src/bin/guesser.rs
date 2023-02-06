mod guess_result;
use guess_result::{GuessBody, GuessResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename("/workspaces/only in dreams/bot/personal.env").ok();

    // Get our cookie and csrf token from environment variables
    let cookie = std::env::var("COOKIE").unwrap();
    let csrf_token = std::env::var("CSRF_TOKEN").unwrap();

    // Create reqwest client to make blocking http requests
    let client = reqwest::blocking::Client::new();

    // Example guess
    let guess_body = GuessBody {
        challenge_id: 7,
        submission: "1".into(),
    };

    // Send example guess http request
    let result = client
        .post("https://ctf.acm.umn.edu/api/v1/challenges/attempt")
        .header("csrf-token", csrf_token)
        .header(reqwest::header::COOKIE, cookie)
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&guess_body)
        .send()?
        .json::<GuessResult>()
        .unwrap();

    // Print results
    println!("{:#?}", result);
    Ok(())
}
