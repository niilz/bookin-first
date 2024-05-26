use std::{error::Error, sync::Arc};

use crate::{
    cookies::Cookie,
    dto::{
        course::Course,
        request::{BookingRequest, EgymLoginRequest},
        response::BookingResponse,
        slots::Slot,
    },
    fitness_service::FitnessService,
    http_client::{FetchApiClient, HttpClient},
    login_service::{LoginCreds, LoginService},
};

pub struct CourseResponse {
    pub user_id: usize,
    pub course_options: Vec<Course>,
}

pub struct BookingService<ClientT, CookieT> {
    login_service: LoginService<Arc<ClientT>, Arc<CookieT>>,
    fitness_service: FitnessService<Arc<ClientT>>,
}

impl<ClientT, CookieT> BookingService<ClientT, CookieT>
where
    ClientT: HttpClient,
    CookieT: Cookie,
{
    pub fn new(http_client: ClientT, cookie_jar: CookieT) -> BookingService<ClientT, CookieT> {
        let http_client = Arc::new(http_client);
        let cookie_jar = Arc::new(cookie_jar);

        let login_service = LoginService::new(Arc::clone(&http_client), Arc::clone(&cookie_jar));

        let fitness_service = FitnessService::new(Arc::clone(&http_client));

        Self {
            login_service,
            fitness_service,
        }
    }

    pub async fn login(
        &mut self,
        user_name: &str,
        password: &str,
    ) -> Result<LoginCreds, Box<dyn Error>> {
        let login_request = EgymLoginRequest::new(user_name, password);

        let _response = self.login_service.do_login(login_request).await;

        self.login_service.get_login_credentials()
    }

    pub async fn fetch_courses(&self, credentials: &LoginCreds) -> Vec<Course> {
        self.fitness_service
            .fetch_courses(credentials)
            .await
            .expect("read courses")
    }

    pub async fn fetch_slots(&self, course: &Course, credentials: &LoginCreds) -> Vec<Slot> {
        self.fitness_service
            .fetch_slots(course.id, credentials)
            .await
            .expect("read slots")
    }

    pub async fn book_course(
        &self,
        booking: BookingRequest,
        credentials: LoginCreds,
    ) -> Result<BookingResponse, Box<dyn Error>> {
        self.fitness_service.book_course(booking, credentials).await
    }
}
