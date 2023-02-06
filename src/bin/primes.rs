use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    // We will check the first 10,000 prime numbers starting from the 1st prime
    let num_primes = 10_000;
    let starting_prime_index = 1;
    println!(
        "Generating the next {} primes starting at {}",
        num_primes, starting_prime_index
    );

    // Find upper bound for the primes we are interested in for the sieve
    let (_low, high) = primal::estimate_nth_prime(num_primes);

    // Create a sieve to find the primes up to this upper bound and locks to share
    // the sieve and the prime index between threads.
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html
    let sieve = primal::Sieve::new(high as usize);
    let sieve_lock = Arc::new(Mutex::new(sieve));
    let prime_index_lock = Arc::new(Mutex::new(starting_prime_index));

    // Start a socket at this path
    // https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html
    let socket_path = std::env::var("COOKIE").unwrap_or("prime_sieve.sock".into());
    let unix_listener = UnixListener::bind(socket_path)?;

    // Always listening for incoming connections
    for stream in unix_listener.incoming() {
        match stream {
            // On successful connection to our socket
            Ok(stream) => {
                let sieve_lock = Arc::clone(&sieve_lock);
                let prime_index_lock = Arc::clone(&prime_index_lock);

                // Start thread to handle this stream and join it to our main thread
                std::thread::spawn(move || {
                    // Buffered reader and writer for the stream
                    let stream_reader = BufReader::new(&stream);
                    let mut stream_writer = BufWriter::new(&stream);

                    // When we receive any message through this socket
                    for _line in BufRead::lines(stream_reader) {
                        // Compute the next prime number using the sieve
                        let mut prime_index = prime_index_lock.lock().unwrap();
                        let prime = sieve_lock.lock().unwrap().nth_prime(*prime_index);

                        // Increment the prime number index
                        *prime_index += 1;

                        // Write the prime to the stream
                        stream_writer
                            .write_all(&prime.to_string().as_bytes())
                            .unwrap();
                        stream_writer.flush().unwrap();
                    }
                });
            }

            // On unsuccessful connection to our socket
            Err(error) => panic!("{}", error),
        }
    }

    Ok(())
}
