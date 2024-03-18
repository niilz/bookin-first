use std::error::Error;

use crate::{
    dto::{course::Course, response::Response},
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
            Ok(serde_json::from_str::<Vec<Course>>(&courses_json)
                .expect("Could not deserialize into courses"))
        } else {
            Err(Box::from("Unexpected Response-Type"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        dto::course::Course,
        fitness_service::FitnessService,
        mock_client,
        testutil::{CredentialsMock, MockCall},
    };

    #[tokio::test]
    async fn read_all_courses() {
        mock_client!(
            MockCall::None,
            MockCall::None,
            Some(|| Ok(Response::Json("".to_string())))
        );
        let creds_mock = CredentialsMock;
        let mock_client = HttpClientMock;
        let fitness_service = FitnessService::new(creds_mock, mock_client);
        let courses = fitness_service.read_courses().await.expect("test failed");

        let expected_courses = generate_course_list(5);
        assert_eq!(expected_courses, courses);
    }

    fn generate_course_list(count: u32) -> Vec<Course> {
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
}
