pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
    pub client_id: String,
}
impl LoginRequest {
    pub fn new(user_name: &str, password: &str, client_id: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
            client_id: client_id.to_string(),
        }
    }
}
