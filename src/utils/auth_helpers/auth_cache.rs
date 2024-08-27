use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::types::globals::AuthenticatedUser;

pub struct AuthCache {
    cache: Arc<RwLock<HashMap<i32, (AuthenticatedUser, Instant)>>>,
}

impl AuthCache {
    pub fn new() -> Self {
        AuthCache {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_user(&self, user_id: i32) -> Option<AuthenticatedUser> {
        let cache = self.cache.read().await;
        if let Some((user, timestamp)) = cache.get(&user_id) {
            if timestamp.elapsed() < std::time::Duration::from_secs(600) {
                return Some(user.clone());
            }
        }
        None
    }

    pub async fn store_user(&self, user: AuthenticatedUser) {
        let mut cache = self.cache.write().await;
        cache.insert(user.0.user_id, (user, Instant::now()));
    }
}
