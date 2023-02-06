mod guess_result;
use guess_result::{GuessBody, GuessResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Personal cookie and csrf token
    let cookie = "session=5730efb4-e22d-4cb7-84bb-91f719ed2e87._6iJF5ld8RcbI3sDTUeZbjkoCrA";
    let csrf_token = "e3ba75735a63d52cb34bc1eb2e2eb44730d9fa0ad734841f11fe0d9f312fa0b5";

    // Create reqwest client to make blocking http requests
    let client = reqwest::blocking::Client::new();

    // Sample guess
    let guess_body = GuessBody {
        challenge_id: 7,
        submission: "1".into(),
    };

    // How long to wait between each request
    let mut seconds_between_requests = 7;

    // How many successful requests without being ratelimited
    let mut non_ratelimited_requests = 0;
    println!(
        "Starting out sending {} requests per minute",
        60 / seconds_between_requests
    );

    loop {
        // Send example guess http request
        let response = client
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
        println!("response: {:?}", response);

        // Check for success (banned if none)
        if response.success.is_none() {
            break;
        }

        // Check for rate limit message
        if response.data.unwrap().status == "ratelimited" {
            break;
        }

        // Increase good requests
        non_ratelimited_requests += 1;

        // So many non rate limited requests, can probably go faster!
        if non_ratelimited_requests >= 50 {
            println!(
                "Haven't been rate limited for {} seconds, decreasing seconds between requests from {} to {}",
                non_ratelimited_requests * seconds_between_requests,
                seconds_between_requests,
                seconds_between_requests - 1
            );
            seconds_between_requests -= 1;
            non_ratelimited_requests = 0;
        }

        std::thread::sleep(std::time::Duration::from_secs(seconds_between_requests));
    }

    println!("Was able to send {} requests per minute by waiting {} seconds between requests for {} minutes without getting rate limited", 60.0 / (seconds_between_requests + 1) as f32, seconds_between_requests + 1, (seconds_between_requests + 1) * 50);
    Ok(())
}
