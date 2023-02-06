mod attempt;
mod create_team;
mod get_nonce_for_page;
mod login;
mod ratelimit;
mod register;
mod types;

pub use crate::attempt::attempt;
pub use crate::create_team::create_team;
pub use crate::get_nonce_for_page::get_nonce_for_page;
pub use crate::login::login;
pub use crate::ratelimit::{MAX_REQUESTS_PER_MINUTE, RATELIMIT_SLEEP_DURATION};
pub use crate::register::register;
pub use crate::types::{GuessBody, GuessResult, GuessResultData, TeamCredentials, UserCredentials};

pub fn get_random_user_and_team_credentials() -> std::io::Result<(UserCredentials, TeamCredentials)>
{
    let user_credentials = UserCredentials::random_credentials();
    let team_credentials = TeamCredentials::team_for_user(&user_credentials);
    Ok((user_credentials, team_credentials))
}
