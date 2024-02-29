use std::error::Error;

use crate::{http_client::HttpClient, request::LoginRequest};

impl<Client> LoginService<Client>
where
    Client: HttpClient,
{
    pub async fn do_login(&mut self, request: LoginRequest) -> Result<(), Box<dyn Error>> {
        match self.http_client.do_login(request).await {
            Ok(res) => {
                println!("Login succeeded");
                self.token = Some(res.session_token);
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
    use crate::response::LoginResponse;

    use super::*;

    #[derive(Default, Debug)]
    struct HttpClientMock;
    impl HttpClient for HttpClientMock {
        async fn do_login(&self, _req: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
            Ok(LoginResponse {
                session_token: "session:12345".to_string(),
            })
        }
    }

    #[derive(Default, Debug)]
    struct FailingHttpClientMock;
    impl HttpClient for FailingHttpClientMock {
        async fn do_login(&self, _req: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
            Err(Box::from("Failed as planned for test"))
        }
    }

    #[tokio::test]
    async fn setup_service() {
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = LoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_ok());
        assert!(login_service.token.is_some());
        assert_eq!("session:12345", login_service.token.unwrap());
    }

    #[tokio::test]
    async fn setup_fails() {
        let mut login_service: LoginService<FailingHttpClientMock> = Default::default();
        let req = LoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_none());
    }
}
