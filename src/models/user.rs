use serde::{Deserialize, Serialize};

/// What we save is internal, we dont Serialize to make it impossible
/// to include in a response.
#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
}

/// What the client send us in the POST request to /register
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// What we pass in the response to the client without the hash
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
}

impl From<&User> for UserResponse {
    fn from(u: &User) -> Self {
        Self {
            id: u.id,
            username: u.username.clone(),
        }
    }
}
