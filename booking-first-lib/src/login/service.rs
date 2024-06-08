use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    dto::{error::BoxDynError, request::EgymLoginRequest, response::Response},
    http_client::HttpClientSend,
    login::parse::extract_session,
};

use super::parse::extract_user_id;

#[derive(Debug, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct LoginCreds {
    pub session: String,
    pub user_id: usize,
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
    pub async fn do_login(&self, request: EgymLoginRequest) -> Result<LoginCreds, BoxDynError> {
        let jwt_token = match self.http_client.egym_login(request).await {
            Ok(Response::Text(res)) => {
                if let Some((_, token)) = res.rsplit_once("?token=") {
                    println!("Egym-RES: {res}");
                    println!("Egym login succeeded");
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
        Ok(LoginCreds {
            session,
            user_id: extract_user_id(&jwt_token)?,
        })
    }

    async fn login_to_fitness_first(&self, token: &str) -> Result<String, BoxDynError> {
        match self.http_client.ff_login(token).await {
            Ok(Response::Cookies(cookies)) => {
                println!("FF login succeeded. PHPSESSID-Cookie should be in Jar");
                let session_cookie = extract_session(cookies /*FITNESS_FIRST_BASE_URL*/);
                println!("PHPSESSID: {session_cookie}");
                Ok(session_cookie)
            }
            Ok(_) => Err(Box::from("unexpected response type")),
            Err(e) => Err(Box::from(format!("login fitness-first failed: {e}"))),
        }
    }
}

#[cfg(test)]

mod test {

    use crate::{
        dto::request::EgymLoginRequest,
        login::service::LoginService,
        mock_client,
        testutil::{egym_login_response_dummy, ff_login_response_dummy},
    };

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
            Some(Err(Box::from(ff_login_err))),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );

        let login_service = LoginService::new(http_client_mock);
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

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
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

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
            Some(ff_login_response_dummy(COOKIES_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = EgymLoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req).await;

        assert!(login_creds.is_ok());
        let login_creds = login_creds.unwrap();
        assert_eq!(login_creds.session, COOKIES_DUMMY);
        assert_eq!(login_creds.user_id, 1234567890);
    }

    #[tokio::test]
    async fn ff_login_fails() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            Some(Err(Box::from(FF_LOGIN_ERR_DUMMY))),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = EgymLoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req).await;

        assert!(login_creds.is_err());
    }

    #[tokio::test]
    async fn can_decode_user_id_from_token() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            Some(ff_login_response_dummy(COOKIES_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let login_service = LoginService::new(http_client_mock);
        let req = EgymLoginRequest::new("user", "password");
        let login_creds = login_service.do_login(req).await;

        assert!(login_creds.is_ok());
        let login_creds = login_creds.unwrap();
        assert_eq!(login_creds.user_id, 1234567890);
        assert_eq!(login_creds.session, COOKIES_DUMMY);
    }
}
