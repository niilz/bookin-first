use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SlotsResult {
    /// Don't really care about this but I don't know how to ignore it
    #[serde(rename = "classes")]
    inner: Slots,
}

impl FromIterator<Slot> for SlotsResult {
    fn from_iter<T: IntoIterator<Item = Slot>>(iter: T) -> Self {
        Self {
            inner: Slots {
                slots: iter.into_iter().collect(),
            },
        }
    }
}

impl SlotsResult {
    pub fn slots(self) -> Vec<Slot> {
        self.inner.into_iter().collect()
    }
}

// This is what we actually want to work with but because the json is so nested
// we patch everything through from the outer `SlotResult` wrapper
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Slots {
    #[serde(rename = "result")]
    slots: Vec<Slot>,
}

impl IntoIterator for Slots {
    type Item = Slot;
    type IntoIter = <Vec<Slot> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Slot {
    /// Slot - ID
    pub id: usize,
    /// Time when the this Course-Slot starts
    #[serde(rename = "startDateTime")]
    #[serde(with = "date_format")]
    pub start_date_time: DateTime<Local>, // "2024-03-19T18:00:00+01:00",
    #[serde(rename = "endDateTime")]
    #[serde(with = "date_format")]
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
    #[serde(with = "date_format")]
    pub earliest_booking_date_time: DateTime<Local>, // e.g.: "2024-03-18T18:00:00+01:00",
    #[serde(rename = "latestBookingDateTime")]
    //#[serde(with = "date_format")]
    pub latest_booking_date_time: Option<DateTime<Local>>,
    #[serde(rename = "maxParticipants")]
    pub max_participants: u8,
    #[serde(rename = "maxWaitingListParticipants")]
    pub max_waiting_list_participants: u8,
    #[serde(rename = "bookedParticipants")]
    pub booked_participants: u8,
    #[serde(rename = "waitingListParticipants")]
    pub waiting_list_participants: u8,
}

impl From<Slot> for JsValue {
    fn from(slot: Slot) -> Self {
        let slot_json = serde_json::to_string(&slot).expect("Slot to json");
        JsValue::from_str(&slot_json)
    }
}

mod date_format {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%+";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?
            .and_local_timezone(Local)
            // TODO: Do not unwrap
            .unwrap();
        Ok(dt)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Local, NaiveDate};
    use serde_json::json;

    use crate::dto::slots::SlotsResult;

    #[tokio::test]
    async fn deserialize_into_slot_works() {
        let slots_json = json!({
            "classes": {
                "result": [
                    {
                        "id": 1486091113,
                        "startDateTime": "2024-03-18T18:30:00+01:00",
                        "endDateTime": "2024-03-18T19:30:00+01:00",
                        "instructors": [
                            {
                                "id": 1390790684,
                                "firstName": "Fitness Coach",
                                "lastName": "Fitness First HH3"
                            }
                        ],
                        "location": {
                            "id": 1390788673,
                            "name": "freestyle Fläche",
                            "description": null
                        },
                        "earliestBookingDateTime": "2024-03-17T18:30:00+01:00",
                        "latestBookingDateTime": null,
                        "maxParticipants": 20,
                        "maxWaitingListParticipants": 0,
                        "bookedParticipants": 20,
                        "waitingListParticipants": 0
                    }
                ]
            }
        });

        let deserialized: SlotsResult =
            serde_json::from_value(slots_json).expect("test fails: slot deserserialization");

        let slot = deserialized
            .slots()
            .into_iter()
            .next()
            .expect("test fails: get first slot");

        let expected_date = NaiveDate::from_ymd_opt(2024, 3, 17)
            .unwrap()
            .and_hms_opt(18, 30, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        assert_eq!(expected_date, slot.earliest_booking_date_time)
    }
}
