struct LoginRequest {
    user_name: String,
    password: String,
}
impl LoginRequest {
    fn new(user_name: &str, password: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
        }
    }
}

struct LoginResponse {
    session_token: String,
}

#[derive(Default, Debug)]
struct LoginService<Client> {
    token: Option<String>,
    http_client: Client,
}

trait HttpClient {
    fn do_login(&self, request: LoginRequest) -> Result<LoginResponse, ()>;
}

impl<Client> LoginService<Client>
where
    Client: HttpClient,
{
    fn do_login(&mut self, request: LoginRequest) {
        match self.http_client.do_login(request) {
            Ok(res) => self.token = Some(res.session_token),
            Err(_) => eprintln!("oops"),
        }
    }
}

#[derive(Default, Debug)]
struct HttpClientMock;
impl HttpClient for HttpClientMock {
    fn do_login(&self, _req: LoginRequest) -> Result<LoginResponse, ()> {
        Ok(LoginResponse {
            session_token: "session:12345".to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn setup_service() {
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = LoginRequest::new("user", "password");
        login_service.do_login(req);

        assert!(login_service.token.is_some());
        assert_eq!("session:12345", login_service.token.unwrap());
    }
}
