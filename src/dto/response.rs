use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Response {
    Text(String),
    SessionSet,
    Json(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum BookingState {
    BOOKED,
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
