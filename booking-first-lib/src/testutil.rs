use serde::Serialize;
use shared::dto::{error::BoxDynError, response::Response};

#[macro_export]
macro_rules! mock_client {
    ($egym_dummy:expr,
     $netpulse_dummy:expr,
     $ff_dummy:expr,
     $courses_dummy:expr,
     $slots_dummy:expr,
     $book_dummy:expr) => {{
        use crate::{http_client::HttpClientSend, testutil::MockRes};
        use shared::dto::{
            error::BoxDynError,
            request::{BookingRequest, LoginRequest},
            response::Response,
        };

        #[derive(Default, Debug)]
        struct HttpClientSendMock {
            egym_dummy: MockRes,
            netpulse_dummy: MockRes,
            ff_dummy: MockRes,
            courses_dummy: MockRes,
            slots_dummy: MockRes,
            book_dummy: MockRes,
        }

        impl HttpClientSend for HttpClientSendMock {
            async fn egym_login(&self, _request: LoginRequest) -> Result<Response, BoxDynError> {
                match self.egym_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn netpulse_login(
                &self,
                _request: LoginRequest,
            ) -> Result<Response, BoxDynError> {
                match self.netpulse_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn ff_login(&self, _request: &str) -> Result<Response, BoxDynError> {
                match self.ff_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn fetch_courses(
                &self,
                _session_id: &str,
                _user_id: Option<&str>,
            ) -> Result<Response, BoxDynError> {
                match self.courses_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn fetch_slots(
                &self,
                _course_id: usize,
                _session_id: &str,
            ) -> Result<Response, BoxDynError> {
                match self.slots_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }

            async fn book_course(
                &self,
                _booking: BookingRequest,
                _session_id: &str,
            ) -> Result<Response, BoxDynError> {
                match self.book_dummy.as_ref() {
                    Some(Ok(res)) => Ok(res.clone()),
                    Some(Err(e)) => Err(Box::from(e.to_string())),
                    None => todo!("test failed, unexpected path"),
                }
            }
        }

        let mock = HttpClientSendMock {
            egym_dummy: $egym_dummy,
            netpulse_dummy: $netpulse_dummy,
            ff_dummy: $ff_dummy,
            courses_dummy: $courses_dummy,
            slots_dummy: $slots_dummy,
            book_dummy: $book_dummy,
        };
        mock
    }};
}

pub(crate) type MockRes = Option<Result<Response, BoxDynError>>;

pub(crate) fn egym_login_response_dummy(egym_jwt: &str) -> Result<Response, BoxDynError> {
    Ok(Response::Text(egym_jwt.to_string()))
}

pub(crate) fn netpulse_login_response_dummy(
    login_res: &str,
    session: &str,
) -> Result<Response, BoxDynError> {
    Ok(Response::WithSession {
        response: login_res.to_string(),
        session: session.to_string(),
    })
}

pub(crate) fn ff_login_response_dummy(session: &str) -> Result<Response, BoxDynError> {
    Ok(Response::Session(session.to_string()))
}

pub(crate) fn serialize_response_dummy(
    response_data: impl Serialize,
) -> Result<Response, BoxDynError> {
    let response: String =
        serde_json::to_string(&response_data).expect("test: serialize expected response-data");
    Ok(Response::Json(response))
}
