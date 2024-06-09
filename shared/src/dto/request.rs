use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct BookingRequest {
    #[serde(rename = "customerId")]
    user_id: usize,
    #[serde(rename = "classSlotId")]
    slot_id: usize,
    #[serde(rename = "classId")]
    course_id: usize,
    #[serde(rename = "clubId")]
    club_id: String,
    #[serde(rename = "clubName")]
    club_name: String,
    #[serde(rename = "className")]
    course_name: String,
}

impl BookingRequest {
    pub fn new(user_id: usize, slot_id: usize, course_id: usize, course_name: String) -> Self {
        Self {
            user_id,
            slot_id,
            course_id,
            club_id: "hamburg3".to_string(),
            club_name: "Hamburg - Eppendorf".to_string(),
            course_name,
        }
    }
}
