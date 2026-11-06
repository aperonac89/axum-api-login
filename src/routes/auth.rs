use crate::{
    auth::password::hash_password,
    error::{AppError, Result},
    models::user::{RegisterRequest, User, UserResponse},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::atomic::Ordering;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    // 1. Validar(TODO: usuario no vacio, contraseña con longitud minima)
    let RegisterRequest { username, password } = payload;
    let username = username.trim().to_string();

    if username.is_empty() || password.is_empty() {
        return Err(AppError::BadRequest(
            "error: username or password are not provided".into(),
        ));
    }

    if password.chars().count() < 8 {
        return Err(AppError::BadRequest(
            "error: password doesnt met the condition(at least 8 characters".into(),
        ));
    }

    if password.chars().count() > 128 {
        return Err(AppError::BadRequest(
            "error: pasword doesnt met the condition(more than 128 characters".into(),
        ));
    }
    // 2. Hashear antes de coger el lock
    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|e| AppError::Internal(format!("task join: {e}")))??;

    // 3. Coger el lock de escritura
    let mut users = state
        .users
        .write()
        .map_err(|_| AppError::Internal("user lock poisoned".into()))?;
    // 4. Comprobar e insertar en bajo el mismo lock
    if users.contains_key(&username) {
        return Err(AppError::UserExists);
    }

    // 5. Crear el user con id = state.next_id.fetch_add(1, Ordering::Relaxed)
    let id = state.next_id.fetch_add(1, Ordering::Relaxed) + 1;
    let user = User {
        id,
        username: username.clone(),
        password_hash,
    };

    let response = UserResponse::from(&user);
    users.insert(username, user);

    // TODO: Insertar y devolver(StatusCode::CREATED, Json(UserResponse::from(&user)))
    Ok((StatusCode::CREATED, Json(response)))
}
