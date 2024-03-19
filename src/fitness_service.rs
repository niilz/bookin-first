use std::error::Error;

use crate::{
    dto::{
        course::{Course, CoursesResult},
        response::Response,
        slots::{Slot, SlotsResult},
    },
    http_client::HttpClient,
    login_service::LoginCreds,
};

pub struct FitnessService<Creds, Client> {
    credendials: Creds,
    http_client: Client,
}

impl<Creds, Client> FitnessService<Creds, Client>
where
    Creds: LoginCreds,
    Client: HttpClient,
{
    pub fn new(credendials: Creds, http_client: Client) -> Self {
        Self {
            credendials,
            http_client,
        }
    }
    pub async fn read_courses(&self) -> Result<Vec<Course>, Box<dyn Error>> {
        let courses_res = self
            .http_client
            .read_courses(&self.credendials.get_session_id().unwrap())
            .await?;
        if let Response::Json(courses_json) = courses_res {
            let result = serde_json::from_str::<CoursesResult>(&courses_json)
                .expect("Could not deserialize into courses");
            Ok(result.courses)
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }
    pub async fn read_slots(&self, course_id: usize) -> Result<Vec<Slot>, Box<dyn Error>> {
        let slots_res = self
            .http_client
            .read_slots(course_id, &self.credendials.get_session_id().unwrap())
            .await?;
        if let Response::Json(slots_json) = slots_res {
            let result = serde_json::from_str::<SlotsResult>(&slots_json)
                .expect("Could not deserialize into slots");
            Ok(result.slots())
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Local};

    use crate::{
        dto::{
            course::{Course, CoursesResult},
            slots::{Slot, SlotsResult},
        },
        fitness_service::FitnessService,
        mock_client,
        testutil::{serialize_response_dummy, CredentialsMock},
    };

    #[tokio::test]
    async fn read_all_courses_works() {
        let expected_courses = generate_dummy_courses(5);
        let http_client_mock = mock_client!(
            MockRes::None,
            MockRes::None,
            Some(serialize_response_dummy(&expected_courses)),
            MockRes::None
        );

        let creds_mock = CredentialsMock;
        let fitness_service = FitnessService::new(creds_mock, http_client_mock);
        let courses = fitness_service
            .read_courses()
            .await
            .expect("test: read_courses");

        assert_eq!(expected_courses.courses, courses);
    }

    #[tokio::test]
    async fn read_all_slots_works() {
        let expected_slots = generate_dummy_slots(5);
        let http_client_mock = mock_client!(
            MockRes::None,
            MockRes::None,
            MockRes::None,
            Some(serialize_response_dummy(&expected_slots))
        );

        let creds_mock = CredentialsMock;
        let fitness_service = FitnessService::new(creds_mock, http_client_mock);
        let slots = fitness_service
            .read_slots(1234)
            .await
            .expect("test: read_courses");

        assert_eq!(expected_slots.slots(), slots);
    }

    fn generate_dummy_courses(count: u32) -> CoursesResult {
        (0..count)
            .map(|id| Course {
                id: id as usize,
                title: fakeit::words::sentence(5),
                typ: fakeit::words::word(),
                duration: fakeit::datetime::minute().parse::<u32>().unwrap(),
                category: fakeit::words::word(),
                description: fakeit::hipster::sentence(10),
                image_url: fakeit::image::url(42, 42),
                bookable: fakeit::bool_rand::bool(),
            })
            .collect()
    }

    fn convert(fake_date: fakeit::datetime::DateTime) -> DateTime<Local> {
        DateTime::from_timestamp(fake_date.secs, fake_date.nsecs)
            .expect("test fails: convert fake-date to chrono-date")
            .with_timezone(&Local)
    }

    fn generate_dummy_slots(count: u32) -> SlotsResult {
        (0..count)
            .map(|id| Slot {
                id: id as usize,
                start_date_time: convert(fakeit::datetime::date()),
                end_date_time: convert(fakeit::datetime::date()),
                earliest_booking_date_time: convert(fakeit::datetime::date()),
                latest_booking_date_time: None,
                max_participants: count as u8,
                max_waiting_list_participants: 0,
                booked_participants: 0,
                waiting_list_participants: 0,
            })
            .collect()
    }
}
