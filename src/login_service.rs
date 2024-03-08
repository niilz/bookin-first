use std::error::Error;

use crate::{
    http_client::HttpClient,
    request::{EgymLoginRequest, FitnessFirstLoginRequest},
    response::Response,
};

impl<Client> LoginService<Client>
where
    Client: HttpClient,
{
    pub async fn do_login(&mut self, request: EgymLoginRequest) -> Result<(), Box<dyn Error>> {
        if self.token.is_none() {
            match self.http_client.egym_login(request).await {
                Ok(Response::Text(res)) => {
                    if let Some((_, token)) = res.rsplit_once("?token=") {
                        self.token = Some(token.to_string());
                        println!("Egym login succeeded");
                    }
                    eprintln!("Egym login failed");
                    return Err(Box::from(format!("Could not extract token from: {res}")));
                }
                Err(e) => return Err(Box::from(format!("login egym failed: {e}"))),
            };
        }
        self.login_to_fitnes_first().await
    }

    async fn login_to_fitnes_first(&mut self) -> Result<(), Box<dyn Error>> {
        let ff_login_req = FitnessFirstLoginRequest::new(&self.token.as_ref().unwrap());
        match self.http_client.ff_login(ff_login_req).await {
            Ok(_res) => {
                //self.session = Some(res.session_token);
                println!("FF login succeeded");
                Ok(())
            }
            Err(e) => Err(Box::from(format!("login fitness-first failed: {e}"))),
        }
    }
}

#[derive(Default, Debug)]
pub struct LoginService<Client> {
    pub http_client: Client,
    token: Option<String>,
    session: Option<String>,
}
impl<Client> LoginService<Client> {
    pub fn new(http_client: Client) -> Self {
        Self {
            http_client,
            token: None,
            session: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{
        http_client::HttpClient,
        login_service::LoginService,
        mock_client,
        request::{EgymLoginRequest, FitnessFirstLoginRequest},
        response::Response,
        testutil::{egym_login_response_dummy, ff_login_response_dummy, MockCall},
    };

    const EGYM_JWT_DUMMY: &str = "https://www.foo.de/my-area?token=base64jwt";
    const SESS_ID_DUMMY: &str = "PHPSESSID-12345";
    const EGYM_LOGIN_ERR_DUMMY: &str = "Egym login test-failure";
    const FF_LOGIN_ERR_DUMMY: &str = "FF login test-failure";

    #[tokio::test]
    async fn egym_login_success() {
        mock_client!(
            Some(|| egym_login_response_dummy(EGYM_JWT_DUMMY)),
            Some(|| Err(Box::from("FF-login not tested here")))
        );

        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_some());
        assert_eq!("base64jwt", login_service.token.unwrap());
    }

    #[tokio::test]
    async fn egym_login_fails() {
        mock_client!(
            Some(|| Err(Box::from(EGYM_LOGIN_ERR_DUMMY))),
            MockCall::None
        );
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_none());
    }

    #[tokio::test]
    async fn ff_login_success() {
        mock_client!(
            Some(|| egym_login_response_dummy(EGYM_JWT_DUMMY)),
            Some(|| ff_login_response_dummy(SESS_ID_DUMMY))
        );
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_ok());
        assert!(login_service.token.is_some());
        assert!(login_service.session.is_some());
        assert_eq!(SESS_ID_DUMMY, login_service.session.unwrap());
    }

    #[tokio::test]
    async fn ff_login_fails() {
        mock_client!(
            Some(|| egym_login_response_dummy(EGYM_JWT_DUMMY)),
            Some(|| Err(Box::from(FF_LOGIN_ERR_DUMMY)))
        );
        let mut login_service: LoginService<HttpClientMock> = Default::default();
        let req = EgymLoginRequest::new("user", "password", "client-id");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_some());
        assert_eq!(EGYM_JWT_DUMMY, login_service.token.unwrap());
        assert!(login_service.session.is_none());
    }
}
