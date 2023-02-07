use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    let num_primes = std::env::var("NUM_PRIMES")
        .map(|p| p.parse::<u64>().unwrap())
        .unwrap_or(12);

    let starting_prime_index = std::env::var("STARTING_PRIME_INDEX")
        .map(|i| i.parse::<usize>().unwrap())
        .unwrap_or(10);

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
    let socket_path =
        std::env::var("SOCKET_LOCATION").unwrap_or_else(|_| "prime_sieve.sock".into());
    std::fs::remove_file(&socket_path).ok();
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
                        let mut prime_index = prime_index_lock.lock().unwrap();

                        // Exit condition
                        if *prime_index > num_primes.try_into().unwrap() {
                            stream_writer.flush().unwrap();
                            break;
                        }

                        // Compute the next prime number using the sieve
                        let prime = sieve_lock.lock().unwrap().nth_prime(*prime_index);

                        // Increment the prime number index
                        *prime_index += 1;

                        // Write the prime to the stream
                        stream_writer.write_all(&prime.to_ne_bytes()).unwrap();
                        stream_writer.flush().unwrap();
                    }

                    // Try to cleanup
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                });
            }

            // On unsuccessful connection to our socket
            Err(error) => panic!("{}", error),
        }
    }

    Ok(())
}
