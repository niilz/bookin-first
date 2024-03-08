use std::error::Error;

use crate::response::{EgymLoginResponse, FitnessFirstLoginResponse};

#[macro_export]
macro_rules! mock_client {
    ($egym_dummy:expr, $ff_dummy:expr) => {
        #[derive(Default, Debug)]
        struct HttpClientMock;

        impl HttpClient for HttpClientMock {
            async fn egym_login(
                &self,
                _request: EgymLoginRequest,
            ) -> Result<EgymLoginResponse, Box<dyn Error>> {
                if let Some(call) = $egym_dummy {
                    call()
                } else {
                    todo!("test failed, unexpected path")
                }
            }

            async fn ff_login(
                &self,
                _request: FitnessFirstLoginRequest,
            ) -> Result<FitnessFirstLoginResponse, Box<dyn Error>> {
                if let Some(call) = $ff_dummy {
                    call()
                } else {
                    todo!("test failed, unexpected path")
                }
            }
        }
    };
}

pub(crate) type MockCall = Option<fn() -> Result<FitnessFirstLoginResponse, Box<dyn Error>>>;

pub(crate) fn egym_login_response_dummy(
    egym_jwt: &str,
) -> Result<EgymLoginResponse, Box<dyn Error>> {
    Ok(EgymLoginResponse {
        egym_jwt: egym_jwt.to_string(),
    })
}

pub(crate) fn ff_login_response_dummy(
    session: &str,
) -> Result<FitnessFirstLoginResponse, Box<dyn Error>> {
    Ok(FitnessFirstLoginResponse {
        session_token: session.to_string(),
    })
}
