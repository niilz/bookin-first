pub struct EgymLoginRequest {
    pub user_name: String,
    pub password: String,
    pub client_id: &'static str,
}

// Aparently the client-id is not per user but per company registered with eGym
const FF_CLIENT_ID: &str = "a175bce7-3e5b-4863-92a1-efc1991ae6fd";

impl EgymLoginRequest {
    pub fn new(user_name: &str, password: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
            client_id: FF_CLIENT_ID,
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

pub struct Booking {
//{`"customerId`":`"1380798137`",`"classSlotId`":1486092405,`"classId`":1355292810,`"clubId`":`"hamburg3`",`"clubName`":`"Hamburg - Eppendorf`",`"className`":`"Hyrox (M/F)`"}
}
