use serde::Serialize;

use crate::http_client::HttpClientSend;
use shared::dto::{
    error::BoxDynError,
    request::LoginRequest,
    response::{NetpulseLoginResponse, Response},
};

use super::parse::extract_user_id;

#[derive(Debug, Serialize)]
pub struct LoginCreds {
    pub session: String,
    pub user_id: UserId,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub enum UserId {
    Num(usize),
    Uuid(String),
}

#[derive(Default, Debug)]
pub struct LoginService<ClientT> {
    pub http_client: ClientT,
}
impl<ClientT> LoginService<ClientT>
where
    ClientT: HttpClientSend,
{
    pub fn new(http_client: ClientT) -> Self {
        Self { http_client }
    }
}

impl<ClientT> LoginService<ClientT>
where
    ClientT: HttpClientSend,
{
    pub async fn do_login(
        &self,
        request: LoginRequest,
        mode: &str,
    ) -> Result<LoginCreds, BoxDynError> {
        match mode {
            "web" => self.login_web(request).await,
            "app" => self.login_app(request).await,
            unsupported_mode => {
                let e = format!("unsupported mode: '{unsupported_mode}'");
                Err(Box::from(e))
            }
        }
    }

    async fn login_app(&self, request: LoginRequest) -> Result<LoginCreds, BoxDynError> {
        let (session, login_response) = match self.http_client.netpulse_login(request).await {
            Ok(Response::WithSession { response, session }) => (
                session,
                serde_json::from_str::<NetpulseLoginResponse>(&response)?,
            ),
            Ok(_) => return Err(Box::from("Unexpected Response type for netpulse-login")),
            Err(e) => return Err(Box::from(format!("login netpulse failed: {e}"))),
        };
        Ok(LoginCreds {
            session,
            user_id: UserId::Uuid(login_response.user_id),
        })
    }

    async fn login_web(&self, request: LoginRequest) -> Result<LoginCreds, BoxDynError> {
        let jwt_token = match self.http_client.egym_login(request).await {
            Ok(Response::Text(res)) => {
                if let Some((_, token)) = res.rsplit_once("?token=") {
                    dbg!("Egym-RES: {res}");
                    dbg!("Egym login succeeded");
                    token.to_string()
                } else {
                    eprintln!("Egym login failed");
                    return Err(Box::from(format!("Could not extract token from: {res}")));
                }
            }
            Ok(_) => return Err(Box::from("Unexpected Response type for egym-login")),
            Err(e) => return Err(Box::from(format!("login egym failed: {e}"))),
        };
        let session = self.login_to_fitness_first(&jwt_token).await?;

        let user_id = extract_user_id(&jwt_token)?;
        Ok(LoginCreds {
            session,
            user_id: UserId::Num(user_id),
        })
    }

    async fn login_to_fitness_first(&self, token: &str) -> Result<String, BoxDynError> {
        match self.http_client.ff_login(token).await {
            Ok(Response::Session(session_id)) => {
                dbg!("FF login succeeded. PHPSESSID-Cookie should be in Jar");
                dbg!("PHPSESSID: {session_id}");
                Ok(session_id)
            }
            Ok(_) => Err(Box::from("unexpected response type")),
            Err(e) => Err(Box::from(format!("login fitness-first failed: {e}"))),
        }
    }
}

#[cfg(test)]

mod test {

    use crate::{
        login::service::{LoginService, UserId},
        mock_client,
        testutil::{
            egym_login_response_dummy, ff_login_response_dummy, netpulse_login_response_dummy,
        },
    };
    use shared::dto::{
        request::LoginRequest,
        response::{self, NetpulseLoginResponse},
    };

    const NETPULSE_LOGIN_DUMMY: &str = "Egym login test-failure";
    const EGYM_TOKEN_URL_DUMMY: &str = "https://www.foo.de/my-area?token=";
    const EGYM_JWT_DUMMY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJkdW1teS1pc3N1ZXIiLCJhdWQiOiJkdW1teS1hdWRpZW5jZSIsImV4cCI6MTcxMTc0ODkyNCwiaWF0IjoxNzExNzQ1MzI0LCJzdWIiOiJkdW1teS1zdWIiLCJ1aWQiOiJhMTc1YmNlNy0zZTViLTQ4NjMtOTJhMS1lZmMxOTkxYWU2ZmQ6ZWZnaTVlaDVwd2lqIiwiY2xhaW1zIjp7ImJyYW5kSWQiOiJkdW1teS1icmFuZC1pZCIsImVneW1BY2NvdW50SWQiOiJkdW1teS1lZ3ltLWFjY291bnQtaWQiLCJtZW1iZXJzaGlwSWQiOiJkdW1teS1tZW1iZXJzaGlwLWlkIiwibW1zTWVtYmVyc2hpcElkcyI6WyIxMjM0NTY3ODkwIl19fQ.C_NkEF_U8PNPfSSX_P-aYZdssOygvhz3Q8QEGfbEnkI";
    const COOKIES_DUMMY: &str = "Session: PHPSESSID123DUMMY, Foo: OtherCookie";
    const EGYM_LOGIN_ERR_DUMMY: &str = "Egym login test-failure";
    const FF_LOGIN_ERR_DUMMY: &str = "FF login test-failure";

    #[tokio::test]
    async fn egym_login_success() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let ff_login_err = "Out of scope ERR, only egym-login is testet";
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            MockRes::None,
            Some(Err(Box::from(ff_login_err))),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );

        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let success = login_service.do_login(req, "web").await;

        assert!(success.is_err());
        if let Err(err) = success {
            assert!(err.to_string().ends_with(ff_login_err));
        };
    }

    #[tokio::test]
    async fn egym_login_fails() {
        let http_client_mock = mock_client!(
            Some(Err(Box::from(EGYM_LOGIN_ERR_DUMMY))),
            MockRes::None,
            MockRes::None,
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let success = login_service.do_login(req, "web").await;

        assert!(success.is_err());
        if let Err(err) = success {
            assert!(err.to_string().starts_with("login egym failed"));
        };
    }

    #[tokio::test]
    async fn ff_login_success() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            MockRes::None,
            Some(ff_login_response_dummy(COOKIES_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req, "web").await;

        assert!(login_creds.is_ok());
        let login_creds = login_creds.unwrap();
        assert_eq!(login_creds.session, COOKIES_DUMMY);
        assert_eq!(login_creds.user_id, UserId::Num(1234567890));
    }

    #[tokio::test]
    async fn ff_login_fails() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            MockRes::None,
            Some(Err(Box::from(FF_LOGIN_ERR_DUMMY))),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req, "web").await;

        assert!(login_creds.is_err());
    }

    #[tokio::test]
    async fn can_decode_user_id_from_token() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            MockRes::None,
            Some(ff_login_response_dummy(COOKIES_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req, "web").await;

        assert!(login_creds.is_ok());
        let login_creds = login_creds.unwrap();
        assert_eq!(login_creds.user_id, UserId::Num(1234567890));
        assert_eq!(login_creds.session, COOKIES_DUMMY);
    }

    // App-mode
    #[tokio::test]
    async fn can_login_to_netpulse() {
        let dummy_id = "123-456-789".to_string();
        let mut login_res_dummy = NetpulseLoginResponse::default();
        login_res_dummy.user_id = dummy_id.clone();
        let login_res_dummy = serde_json::to_string(&login_res_dummy).unwrap();

        let session_dummy = "JSESSIONID=FooBarBaz";
        let http_client_mock = mock_client!(
            MockRes::None,
            Some(netpulse_login_response_dummy(
                &login_res_dummy,
                &session_dummy
            )),
            MockRes::None,
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = LoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req, "app").await;

        assert!(login_creds.is_ok());
        let login_creds = login_creds.unwrap();
        assert_eq!(login_creds.user_id, UserId::Uuid(dummy_id));
        assert_eq!(login_creds.session, session_dummy);
    }
}
