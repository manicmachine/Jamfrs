use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
    pub expires: u64,
}
