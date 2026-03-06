use konto_common::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub async fn check(&self, key: &str) -> Result<(), AppError> {
        let mut map = self.requests.lock().await;
        let now = Instant::now();

        map.retain(|_, timestamps| {
            timestamps.retain(|t| now.duration_since(*t) < self.window);
            !timestamps.is_empty()
        });
        let timestamps = map.entry(key.to_string()).or_default();

        if timestamps.len() >= self.max_requests {
            return Err(AppError::BadRequest(
                "Too many requests. Please try again later.".to_string(),
            ));
        }

        timestamps.push(now);
        Ok(())
    }
}
