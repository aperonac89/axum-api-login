use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::ErrorKind;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidCredentials,
    MissingToken,
    InvalidToken,
    UserExists,
    Internal(String),
    BadRequest(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "invalid credentials"),
            Self::MissingToken => write!(f, "missing token"),
            Self::InvalidToken => write!(f, "invalid token"),
            Self::UserExists => write!(f, "user already exists"),
            Self::Internal(msg) => write!(f, "internal error: {}", msg),
            Self::BadRequest(msg) => write!(f, "bad request: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::Internal(format!("password hashing: {e}"))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            ErrorKind::ExpiredSignature
            | ErrorKind::ImmatureSignature
            | ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::MissingRequiredClaim(_) => Self::InvalidToken,
            _ => Self::Internal(format!("jwt: {e}")),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid credentials"),
            Self::MissingToken => (StatusCode::UNAUTHORIZED, "missing token"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
            Self::UserExists => (StatusCode::CONFLICT, "user already exists"),
            Self::Internal(detail) => {
                eprintln!("internal error: {detail}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            Self::BadRequest(detail) => {
                eprintln!("bad request: {detail}");
                (StatusCode::BAD_REQUEST, "bas request")
            }
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
