use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    println!("Starting guesser with user credentials: {:#?}", user);
    only_in_dreams::login(&client, &user).unwrap();

    let mut stream = UnixStream::connect("/var/run/primes/prime_sieve.sock").unwrap();
    let mut buffer: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];

    loop {
        stream.write_all(b".\n").unwrap();
        stream.flush().unwrap();
        stream.read_exact(&mut buffer).unwrap();

        let result = only_in_dreams::attempt(&client, usize::from_ne_bytes(buffer)).unwrap();
        println!("{:#?}", result);
        std::thread::sleep(only_in_dreams::RATELIMIT_SLEEP_DURATION);
    }
}
