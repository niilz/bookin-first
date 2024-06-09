use std::sync::Arc;

use crate::{
    dto::{
        course::Course,
        error::BoxDynError,
        request::{BookingRequest, EgymLoginRequest},
        response::BookingResponse,
        slots::Slot,
    },
    fitness_service::FitnessService,
    http_client::HttpClientSend,
    login::service::{LoginCreds, LoginService},
};

pub struct CourseResponse {
    pub user_id: usize,
    pub course_options: Vec<Course>,
}

pub struct BookingService<ClientT> {
    login_service: LoginService<Arc<ClientT>>,
    fitness_service: FitnessService<Arc<ClientT>>,
}

impl<ClientT> BookingService<ClientT>
where
    ClientT: HttpClientSend,
{
    pub fn new(http_client: ClientT) -> BookingService<ClientT> {
        let http_client = Arc::new(http_client);

        let login_service = LoginService::new(Arc::clone(&http_client));

        let fitness_service = FitnessService::new(Arc::clone(&http_client));

        Self {
            login_service,
            fitness_service,
        }
    }

    pub async fn login(&self, user_name: &str, password: &str) -> Result<LoginCreds, BoxDynError> {
        let login_request = EgymLoginRequest::new(user_name, password);

        self.login_service.do_login(login_request).await
    }

    pub async fn fetch_courses(&self, session: &str) -> Vec<Course> {
        self.fitness_service
            .fetch_courses(session)
            .await
            .expect("read courses")
    }

    pub async fn fetch_slots(&self, course: &Course, credentials: &LoginCreds) -> Vec<Slot> {
        self.fitness_service
            .fetch_slots(course.id, &credentials.session)
            .await
            .expect("read slots")
    }

    pub async fn book_course(
        &self,
        booking: BookingRequest,
        credentials: &LoginCreds,
    ) -> Result<BookingResponse, BoxDynError> {
        self.fitness_service.book_course(booking, credentials).await
    }
}
