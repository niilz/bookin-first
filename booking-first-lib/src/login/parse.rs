use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

use shared::dto::{
    error::BoxDynError,
    token::{Claims, Jwt},
};

pub fn extract_user_id(jwt_token: &str) -> Result<String, BoxDynError> {
    let payload = jwt_token.split('.').nth(1).ok_or("Payload missing")?;
    let decoded = STANDARD_NO_PAD.decode(payload)?;
    let Jwt {
        claims: Claims { user_ids },
    } = serde_json::from_slice(&decoded)?;
    Ok(user_ids[0].to_string())
}
