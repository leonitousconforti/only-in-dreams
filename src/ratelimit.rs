pub static MAX_REQUESTS_PER_MINUTE: u64 = 10;
pub static RATELIMIT_SLEEP_DURATION: std::time::Duration =
    std::time::Duration::from_secs(60 / MAX_REQUESTS_PER_MINUTE);
