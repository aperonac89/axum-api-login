use crate::error::Result;
use argon2::{Argon2, password_hash::PasswordHasher};

pub fn hash_password(password: &str) -> Result<String> {
    let hash = Argon2::default().hash_password(password.as_bytes())?;
    Ok(hash.to_string())
}
