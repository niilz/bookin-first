use std::collections::HashMap;

const EGYM_LOGIN_URL: &str = "https://id.egym.com/login";

pub struct LoginRequest {
    user_name: String,
    password: String,
}
impl LoginRequest {
    pub fn new(user_name: &str, password: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct LoginResponse {
    session_token: String,
}

#[derive(Default, Debug)]
pub struct LoginService<Client> {
    pub token: Option<String>,
    pub http_client: Client,
}

pub trait HttpClient {
    async fn do_login(&self, request: LoginRequest) -> Result<LoginResponse, ()>;
}

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

pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl HttpClient for ReqwestHttpClient {
    async fn do_login(&self, _request: LoginRequest) -> Result<LoginResponse, ()> {
        let mut params = HashMap::new();
        //params.insert("username", "TODO-ARG-USER");
        //params.insert("password", "TODO-ARG-PWD");
        //params.insert("clientId", "TODO-CLIENT_ID");
        params.insert(
            "callbackUrl",
            "https://www.fitnessfirst.de/mein-fitnessfirst",
        );
        let res = self.client.post(EGYM_LOGIN_URL).form(&params).send().await;
        match res {
            Ok(res) => {
                println!("Response: {res:?}");
                Ok({
                    LoginResponse {
                        session_token: "Dummy".to_string(),
                    }
                })
            }
            Err(e) => {
                eprintln!("oopsi: {e}");
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod test {
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
        let req = LoginRequest::new("user", "password");
        login_service.do_login(req).await;

        assert!(login_service.token.is_some());
        assert_eq!("session:12345", login_service.token.unwrap());
    }
}
