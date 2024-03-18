use std::error::Error;

use crate::{
    cookies::Cookie,
    dto::{course::Course, response::Response},
    login_service::LoginCreds,
};

#[macro_export]
macro_rules! mock_client {
    ($egym_dummy:expr, $ff_dummy:expr, $courses_dummy:expr) => {{
        use crate::{
            dto::{
                request::{EgymLoginRequest, FitnessFirstLoginRequest},
                response::Response,
            },
            http_client::HttpClient,
            testutil::MockRes,
        };
        use std::error::Error;

        #[derive(Default, Debug)]
        struct HttpClientMock {
            egym_dummy: MockRes,
            ff_dummy: MockRes,
            courses_dummy: MockRes,
        }

        impl HttpClient for HttpClientMock {
            async fn egym_login(
                &self,
                _request: EgymLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                match self.egym_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn ff_login(
                &self,
                _request: FitnessFirstLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                match self.ff_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn read_courses(&self, _session_id: &str) -> Result<Response, Box<dyn Error>> {
                match self.courses_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }
        }
        let mock = HttpClientMock {
            egym_dummy: $egym_dummy,
            ff_dummy: $ff_dummy,
            courses_dummy: $courses_dummy,
        };
        mock
    }};
}

pub(crate) type MockRes = Option<Result<Response, Box<dyn Error>>>;

pub(crate) fn egym_login_response_dummy(egym_jwt: &str) -> Result<Response, Box<dyn Error>> {
    Ok(Response::Text(egym_jwt.to_string()))
}

pub(crate) fn ff_login_response_dummy(session: &str) -> Result<Response, Box<dyn Error>> {
    Ok(Response::Text(session.to_string()))
}

pub(crate) fn courses_response_dummy(courses: &Vec<Course>) -> Result<Response, Box<dyn Error>> {
    let courses_str: String =
        serde_json::to_string(courses).expect("test: serialize expected courses");
    Ok(Response::Json(courses_str))
}

#[derive(Default, Debug)]
pub(crate) struct CookieMock;
impl Cookie for CookieMock {
    fn read_cookie(&self, _domain: &str) -> Result<String, Box<dyn Error>> {
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
