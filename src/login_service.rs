use crate::{http_client::HttpClient, request::LoginRequest};

impl<Client> LoginService<Client>
where
    Client: HttpClient,
{
    pub async fn do_login(&mut self, request: LoginRequest) {
        match self.http_client.do_login(request).await {
            Ok(res) => self.token = Some(res.session_token),
            Err(_) => eprintln!("oops"),
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
        async fn do_login(&self, _req: LoginRequest) -> Result<LoginResponse, ()> {
            Ok(LoginResponse {
                session_token: "session:12345".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn setup_service() {
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = LoginRequest::new("user", "password", "client-id");
        login_service.do_login(req).await;

        assert!(login_service.token.is_some());
        assert_eq!("session:12345", login_service.token.unwrap());
    }
}
