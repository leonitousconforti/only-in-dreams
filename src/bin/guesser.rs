fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename("/workspaces/only in dreams/bot/personal.env").ok();

    let email = std::env::var("EMAIL").unwrap_or_default();
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let user = only_in_dreams::UserCredentials {
        email,
        password,
        name: username,
    };

    only_in_dreams::login(&client, &user).unwrap();
    let result = only_in_dreams::attempt(&client, "1".into()).unwrap();
    std::thread::sleep(only_in_dreams::RATELIMIT_SLEEP_DURATION);
    println!("{:#?}", result);
    Ok(())
}
