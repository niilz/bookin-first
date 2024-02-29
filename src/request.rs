pub struct EgymLoginRequest {
    pub user_name: String,
    pub password: String,
    pub client_id: String,
}
impl EgymLoginRequest {
    pub fn new(user_name: &str, password: &str, client_id: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
            client_id: client_id.to_string(),
        }
    }
}

pub struct FitnessFirstLoginRequest {
    pub egym_token: String,
}
impl FitnessFirstLoginRequest {
    pub fn new(egym_token: &str) -> Self {
        Self {
            egym_token: egym_token.to_string(),
        }
    }
}
