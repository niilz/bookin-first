use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SlotsResult {
    #[serde(rename = "result")]
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Slot {
    /// Slot - ID
    pub id: usize,
    /// Time when the this Course-Slot starts
    #[serde(rename = "startDateTime")]
    pub start_date_time: DateTime<Local>, // "2024-03-19T18:00:00+01:00",
    #[serde(rename = "endDateTime")]
    pub end_date_time: DateTime<Local>, // "2024-03-19T18:30:00+01:00",
    /*
    "instructors": [
      {
        "id": 1390790684,
        "firstName": "Fitness Coach",
        "lastName": "Fitness First HH3"
      }
    ],
    "location": {
      "id": 1390788673,
      "name": "freestyle Fl\u00e4che",
      "description": null
    },
    */
    #[serde(rename = "earliestBookingDateTime")]
    pub earliest_booking_date_time: DateTime<Local>, // "2024-03-18T18:00:00+01:00",
    #[serde(rename = "latestBookingDateTime")]
    pub latest_booking_date_time: DateTime<Local>, // null,
    #[serde(rename = "maxParticipants")]
    pub max_participants: u8, // 20,
    #[serde(rename = "maxWaitingListParticipants")]
    pub max_waiting_list_participants: u8, // 0,
    #[serde(rename = "bookedParticipants")]
    pub booked_participants: u8, // 0,
    #[serde(rename = "waitingListParticipants")]
    pub waiting_list_participants: u8, // 0
}
