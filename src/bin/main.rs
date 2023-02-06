fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let user = only_in_dreams::UserCredentials::random_credentials();
    let team = only_in_dreams::TeamCredentials::team_for_user(&user);

    only_in_dreams::register(&client, &user).unwrap();
    only_in_dreams::login(&client, &user).unwrap();
    only_in_dreams::create_team(&client, &team).unwrap();

    let result = only_in_dreams::attempt(&client, "1".into()).unwrap();
    println!("{:#?}", result);
    println!("{:#?}\n{:#?}", user, team);
    Ok(())
}
