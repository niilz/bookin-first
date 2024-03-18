use std::{collections::HashMap, error::Error};

use crate::{cookies::Cookie, dto::response::Response, login_service::LoginCreds};

#[macro_export]
macro_rules! mock_client {
    ($egym_dummy:expr, $ff_dummy:expr, $courses_dummy:expr) => {
        use crate::{
            dto::{
                request::{EgymLoginRequest, FitnessFirstLoginRequest},
                response::Response,
            },
            http_client::HttpClient,
        };
        use std::error::Error;

        #[derive(Default, Debug)]
        struct HttpClientMock;

        fn resolve_call(
            call: Option<fn() -> Result<Response, Box<dyn Error>>>,
        ) -> Result<Response, Box<dyn Error>> {
            if let Some(call) = call {
                call()
            } else {
                todo!("test failed, unexpected path")
            }
        }

        impl HttpClient for HttpClientMock {
            async fn egym_login(
                &self,
                _request: EgymLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                resolve_call($egym_dummy)
            }

            async fn ff_login(
                &self,
                _request: FitnessFirstLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                resolve_call($ff_dummy)
            }

            async fn read_courses(&self, _session_id: &str) -> Result<Response, Box<dyn Error>> {
                resolve_call($courses_dummy)
            }
        }
    };
}

pub(crate) type MockCall<Request> = Option<fn() -> Result<Request, Box<dyn Error>>>;

pub(crate) fn egym_login_response_dummy(egym_jwt: &str) -> Result<Response, Box<dyn Error>> {
    Ok(Response::Text(egym_jwt.to_string()))
}

pub(crate) fn ff_login_response_dummy(session: &str) -> Result<Response, Box<dyn Error>> {
    Ok(Response::Text(session.to_string()))
}

#[derive(Default, Debug)]
pub(crate) struct CookieMock {
    pub(crate) cookie_dummy: HashMap<String, String>,
}
impl Cookie for CookieMock {
    fn read_cookie(&self, domain: &str) -> Result<String, Box<dyn Error>> {
        Ok("PHPSESSID123DUMMY".to_string())
    }
}

#[derive(Default, Debug)]
pub(crate) struct CredentialsMock;
impl LoginCreds for CredentialsMock {
    fn get_session_id(&self) -> Option<String> {
        Some("dummy-session-id".to_string())
    }
}
