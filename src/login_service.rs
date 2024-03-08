use std::error::Error;

use crate::{http_client::HttpClient, request::EgymLoginRequest};

impl<Client> LoginService<Client>
where
    Client: HttpClient,
{
    pub async fn do_login(&mut self, request: EgymLoginRequest) -> Result<(), Box<dyn Error>> {
        match self.http_client.egym_login(request).await {
            Ok(res) => {
                println!("Login succeeded");
                self.token = Some(res.egym_jwt);
                Ok(())
            }
            Err(e) => Err(Box::from(format!("login failed: {e}"))),
        }
    }
}

#[derive(Default, Debug)]
pub struct LoginService<Client> {
    pub token: Option<String>,
    pub http_client: Client,
}

#[cfg(test)]
mod test {
    use crate::{
        request::FitnessFirstLoginRequest,
        response::{EgymLoginResponse, FitnessFirstLoginResponse},
    };

    use super::*;

    #[derive(Default, Debug)]
    struct HttpClientMock;
    impl HttpClient for HttpClientMock {
        async fn egym_login(
            &self,
            _req: EgymLoginRequest,
        ) -> Result<EgymLoginResponse, Box<dyn Error>> {
            Ok(EgymLoginResponse {
                egym_jwt: "session:12345".to_string(),
            })
        }

        async fn ff_login(
            &self,
            request: FitnessFirstLoginRequest,
        ) -> Result<FitnessFirstLoginResponse, Box<dyn Error>> {
            let fitness_first_login_response = FitnessFirstLoginResponse {
                session_token: "PHPSESS-12345".to_string(),
            };
            Ok(fitness_first_login_response)
        }
    }

    #[derive(Default, Debug)]
    struct FailingHttpClientMock;
    impl HttpClient for FailingHttpClientMock {
        async fn egym_login(
            &self,
            _req: EgymLoginRequest,
        ) -> Result<EgymLoginResponse, Box<dyn Error>> {
            Err(Box::from("Failed as planned for test"))
        }

        async fn ff_login(
            &self,
            request: FitnessFirstLoginRequest,
        ) -> Result<FitnessFirstLoginResponse, Box<dyn Error>> {
            todo!()
        }
    }

    #[tokio::test]
    async fn setup_service() {
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_ok());
        assert!(login_service.token.is_some());
        assert_eq!("session:12345", login_service.token.unwrap());
    }

    #[tokio::test]
    async fn setup_fails() {
        let mut login_service: LoginService<FailingHttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_none());
    }
}
