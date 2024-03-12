use std::{collections::HashMap, error::Error};

use crate::{cookies::Cookie, response::Response};

#[macro_export]
macro_rules! mock_client {
    ($egym_dummy:expr, $ff_dummy:expr) => {
        #[derive(Default, Debug)]
        struct HttpClientMock;

        impl HttpClient for HttpClientMock {
            async fn egym_login(
                &self,
                _request: EgymLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                if let Some(call) = $egym_dummy {
                    call()
                } else {
                    todo!("test failed, unexpected path")
                }
            }

            async fn ff_login(
                &self,
                _request: FitnessFirstLoginRequest,
            ) -> Result<Response, Box<dyn Error>> {
                if let Some(call) = $ff_dummy {
                    call()
                } else {
                    todo!("test failed, unexpected path")
                }
            }
            async fn read_courses(&self, _session_id: &str) -> Result<Response, Box<dyn Error>> {
                Ok(Response::Text("Dummy-Course-List".to_string()))
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
