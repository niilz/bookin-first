#[derive(Debug)]
pub struct EgymLoginResponse {
    pub egym_jwt: String,
}

#[derive(Debug)]
pub struct FitnessFirstLoginResponse {
    pub session_token: String,
}
