use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub user_name: String,
    pub password: String,
}
