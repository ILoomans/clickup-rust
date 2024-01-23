use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
    color: Option<String>,
    profilePicture: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedUser {
    user: User,
}
