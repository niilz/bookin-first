use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

use super::course::CourseWithSlot;

#[derive(Debug, Clone)]
pub enum Response {
    Text(String),
    Session(String),
    Json(String),
    WithSession { response: String, session: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum BookingState {
    BOOKED,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Booking {
    App(NetpulseBookingResponse),
    Web(BookingResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookingResponse {
    #[serde(rename = "bookingId")]
    pub booking_id: usize,
    #[serde(rename = "bookingStatus")]
    pub booking_status: BookingState,
    #[serde(rename = "classSlotId")]
    pub slot_id: usize,
    #[serde(rename = "classId")]
    pub course_id: usize,
    #[serde(rename = "customerId")]
    pub customer_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetpulseBookingResponse {
    #[serde(rename = "brief")]
    pub course: CourseWithSlot,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct NetpulseLoginResponse {
    #[serde(rename = "uuid")]
    pub user_id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub verified: bool,
    #[serde(rename = "emailVerified")]
    pub email_varified: bool,
    #[serde(rename = "homeClubUuid")]
    pub club_id: String,
    #[serde(rename = "homeClubName")]
    pub club_name: String,
    #[serde(rename = "chainUuid")]
    pub chain_uuid: String,
    #[serde(rename = "chainName")]
    pub chain_name: String,
    pub timezone: String,
    #[serde(rename = "timezoneOffset")]
    pub timezone_offset: usize,
    #[serde(rename = "profileCompleted")]
    pub profile_completed: bool,
    #[serde(rename = "membershipType")]
    pub membership_type: String,
    //"barcode": null,
    //"externalAuthToken": null,
    //"measurementUnit": "M",
    //"hasMessages": null,
    //"guestPassUser": false,
    //"externalRefreshToken": null,
    //"email": null,
    //"customInfo": null,
    //"newUserSecret": null
}
