use std::error::Error;

use crate::response::Response;

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
