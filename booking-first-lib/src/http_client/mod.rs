pub mod reqwest_client;

use std::sync::Arc;

use crate::dto::{
    error::BoxDynError,
    request::{BookingRequest, EgymLoginRequest, FitnessFirstLoginRequest},
    response::Response,
};

pub const FITNESS_FIRST_BASE_URL: &str = "https://mein.fitnessfirst.de";
pub const FITNESS_FIRST_CALLBACK_URL: &str = "https://www.fitnessfirst.de/mein-fitnessfirst";
pub const EGYM_LOGIN_URL: &str = "https://id.egym.com/login";
pub const EGYM_TOKEN_PATH: &str = "/egymid-login?token=";
pub const COURSES_URL_PATH: &str = "/api/magicline/openapi/classes/hamburg3";

// TODO: Remove when async fn in traits is fully stable (see: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits)
#[trait_variant::make(HttpClientSend: Send + Sync)]
pub trait HttpClient {
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, BoxDynError>;
    async fn ff_login(&self, request: FitnessFirstLoginRequest) -> Result<Response, BoxDynError>;
    async fn fetch_courses(&self, session_id: &str) -> Result<Response, BoxDynError>;
    async fn fetch_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, BoxDynError>;
    async fn book_course(
        &self,
        booking: BookingRequest,
        session_id: &str,
    ) -> Result<Response, BoxDynError>;
}

impl<ClientT> HttpClientSend for Arc<ClientT>
where
    ClientT: HttpClientSend + Sync,
{
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, BoxDynError> {
        self.as_ref().egym_login(request).await
    }

    async fn ff_login(&self, request: FitnessFirstLoginRequest) -> Result<Response, BoxDynError> {
        self.as_ref().ff_login(request).await
    }

    async fn fetch_courses(&self, session_id: &str) -> Result<Response, BoxDynError> {
        self.as_ref().fetch_courses(session_id).await
    }

    async fn fetch_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, BoxDynError> {
        self.as_ref().fetch_slots(course_id, session_id).await
    }

    async fn book_course(
        &self,
        booking: BookingRequest,
        session_id: &str,
    ) -> Result<Response, BoxDynError> {
        self.as_ref().book_course(booking, session_id).await
    }
}
