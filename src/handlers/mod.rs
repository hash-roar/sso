mod auth;
mod shorturls;
mod user;
pub use auth::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
pub use shorturls::*;
pub use user::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SResult {
    error: i32,
    error_msg: String,
    data: String,
}

impl SResult {
    pub fn new<T>(error: i32, err_msg: &str, data: T) -> Self
    where
        T: Serialize,
    {
        SResult {
            error,
            error_msg: err_msg.to_string(),
            data: to_string(&data).ok().unwrap_or_default(),
        }
    }

    pub fn to_string(&self) -> String {
        to_string(&self).map_or("serde error".to_string(), |v| v)
    }
}

impl Default for SResult {
    fn default() -> Self {
        SResult {
            error: 0,
            error_msg: "".to_string(),
            data: "".to_string(),
        }
    }
}

impl Into<String> for SResult {
    fn into(self) -> String {
        to_string(&self).map_or("".to_string(), |v| v)
    }
}
