use crate::models::user::User;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, atomic::AtomicU64},
};

#[derive(Clone, Default)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub next_id: Arc<AtomicU64>,
}
