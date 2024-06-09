use crate::{
    dto::{
        course::{Course, CoursesResult},
        error::BoxDynError,
        request::BookingRequest,
        response::{BookingResponse, Response},
        slots::{Slot, SlotsResult},
    },
    http_client::HttpClientSend,
};

pub struct FitnessService<ClientT> {
    http_client: ClientT,
}

impl<ClientT> FitnessService<ClientT>
where
    ClientT: HttpClientSend,
{
    pub fn new(http_client: ClientT) -> Self {
        Self { http_client }
    }
    pub async fn fetch_courses(&self, session: &str) -> Result<Vec<Course>, BoxDynError> {
        let courses_res = self.http_client.fetch_courses(session).await?;
        if let Response::Json(courses_json) = courses_res {
            let result = serde_json::from_str::<CoursesResult>(&courses_json)
                .expect("Could not deserialize into courses");
            Ok(result.courses)
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }

    pub async fn fetch_slots(
        &self,
        course_id: usize,
        session: &str,
    ) -> Result<Vec<Slot>, BoxDynError> {
        let slots_res = self.http_client.fetch_slots(course_id, session).await?;
        if let Response::Json(slots_json) = slots_res {
            let result = serde_json::from_str::<SlotsResult>(&slots_json)
                .expect("Could not deserialize into slots");
            Ok(result.slots())
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }

    pub async fn book_course(
        &self,
        booking: BookingRequest,
        session: &str,
    ) -> Result<BookingResponse, BoxDynError> {
        let booking_res = self.http_client.book_course(booking, session).await?;
        if let Response::Json(booking_json) = booking_res {
            serde_json::from_str::<BookingResponse>(&booking_json)
                .map_err(serde_json::error::Error::into)
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }
}

#[cfg(test)]
#[cfg(not(target_family = "wasm"))]
mod test {
    use chrono::{DateTime, Local};
    use serde_json::json;

    use crate::{
        dto::{
            course::{Course, CoursesResult},
            request::BookingRequest,
            response::BookingState,
            slots::{Slot, SlotsResult},
        },
        fitness_service::FitnessService,
        mock_client,
        testutil::serialize_response_dummy,
    };

    #[tokio::test]
    async fn read_all_courses_works() {
        let expected_courses = generate_dummy_courses(5);
        let http_client_mock = mock_client!(
            MockRes::None,
            MockRes::None,
            Some(serialize_response_dummy(&expected_courses)),
            MockRes::None,
            MockRes::None
        );

        let session_dummy = "SESSION_DUMMY";
        let fitness_service = FitnessService::new(http_client_mock);
        let courses = fitness_service
            .fetch_courses(&session_dummy)
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
            Some(serialize_response_dummy(&expected_slots)),
            MockRes::None
        );

        let session_dummy = "dummy-session";
        let fitness_service = FitnessService::new(http_client_mock);
        let slots = fitness_service
            .fetch_slots(1234, &session_dummy)
            .await
            .expect("test: read_courses");

        assert_eq!(expected_slots.slots(), slots);
    }

    #[tokio::test]
    async fn can_book_course() {
        let booking_dummy = json!({
        "bookingId": 1234567,
        "bookingStatus": "BOOKED",
        "classSlotId": 89012345,
        "classId": 11223344,
        "customerId": 55667788
              })
        .to_string();
        let http_client_mock = mock_client!(
            MockRes::None,
            MockRes::None,
            MockRes::None,
            MockRes::None,
            Some(Ok(Response::Json(booking_dummy)))
        );

        let session_dummy = "dummy-session";
        let fitness_service = FitnessService::new(http_client_mock);
        let request_dummy = BookingRequest::new(42, 43, 43, "Some Course".to_string());
        let booking = fitness_service
            .book_course(request_dummy, &session_dummy)
            .await
            .expect("test: book course");

        assert_eq!(1234567, booking.booking_id);
        assert_eq!(BookingState::BOOKED, booking.booking_status);
        assert_eq!(89012345, booking.slot_id);
        assert_eq!(11223344, booking.course_id);
        assert_eq!(55667788, booking.customer_id);
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
