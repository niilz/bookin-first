use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use serde::Serialize;
use std::error::Error;
use wasm_bindgen::prelude::*;

use crate::{
    cookies::Cookie,
    dto::{
        request::{EgymLoginRequest, FitnessFirstLoginRequest},
        response::Response,
        token::{Claims, Jwt},
    },
    http_client::{HttpClient, FITNESS_FIRST_BASE_URL},
};

#[derive(Debug, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct LoginCreds {
    pub session: String,
    pub user_id: usize,
}

#[derive(Default, Debug)]
pub struct LoginService<ClientT, CookieT> {
    pub http_client: ClientT,
    token: Option<String>,
    session: Option<String>,
    cookie_jar: CookieT,
}
impl<ClientT, CookieT> LoginService<ClientT, CookieT>
where
    ClientT: HttpClient,
    CookieT: Cookie,
{
    pub fn new(http_client: ClientT, cookie_jar: CookieT) -> Self {
        Self {
            http_client,
            token: None,
            session: None,
            cookie_jar,
        }
    }
}

impl<ClientT, CookieT> LoginService<ClientT, CookieT>
where
    ClientT: HttpClient,
    CookieT: Cookie,
{
    pub async fn do_login(&mut self, request: EgymLoginRequest) -> Result<(), Box<dyn Error>> {
        if self.token.is_none() {
            match self.http_client.egym_login(request).await {
                Ok(Response::Text(res)) => {
                    if let Some((_, token)) = res.rsplit_once("?token=") {
                        self.token = Some(token.to_string());
                        println!("Egym-RES: {res}");
                        println!("Egym login succeeded");
                    } else {
                        eprintln!("Egym login failed");
                        return Err(Box::from(format!("Could not extract token from: {res}")));
                    }
                }
                Ok(_) => return Err(Box::from("Unexpected Response type for egym-login")),
                Err(e) => return Err(Box::from(format!("login egym failed: {e}"))),
            };
        }
        self.login_to_fitness_first().await
    }

    pub fn get_login_credentials(&self) -> Result<LoginCreds, Box<dyn Error>> {
        let payload = match &self.token {
            Some(token) => token.split('.').nth(1).ok_or("Payload missing")?,
            None => return Err("Token missing".into()),
        };
        let decoded = STANDARD_NO_PAD.decode(payload)?;
        let Jwt {
            claims: Claims { user_ids },
        } = serde_json::from_slice(&decoded)?;
        let user_id = user_ids[0].parse::<usize>()?;

        Ok(LoginCreds {
            session: self.session.clone().expect("Session should be here"),
            user_id,
        })
    }

    async fn login_to_fitness_first(&mut self) -> Result<(), Box<dyn Error>> {
        let ff_login_req = FitnessFirstLoginRequest::new(&self.token.as_ref().unwrap());
        match self.http_client.ff_login(ff_login_req).await {
            Ok(_res) => {
                println!("FF login succeeded. PHPSESSID-Cookie should be in Jar");
                let session_cookie = self.cookie_jar.read_cookie(FITNESS_FIRST_BASE_URL)?;
                // TODO: Split session-Id from Cookie key, maybe
                println!("PHPSESSID: {session_cookie}");
                self.session = Some(session_cookie);
                Ok(())
            }
            Err(e) => Err(Box::from(format!("login fitness-first failed: {e}"))),
        }
    }
}

#[cfg(test)]

mod test {

    use std::sync::Arc;

    use crate::{
        dto::request::EgymLoginRequest,
        login_service::LoginService,
        mock_client,
        testutil::{egym_login_response_dummy, ff_login_response_dummy, CookieMock},
    };

    const EGYM_TOKEN_URL_DUMMY: &str = "https://www.foo.de/my-area?token=";
    const EGYM_JWT_DUMMY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJkdW1teS1pc3N1ZXIiLCJhdWQiOiJkdW1teS1hdWRpZW5jZSIsImV4cCI6MTcxMTc0ODkyNCwiaWF0IjoxNzExNzQ1MzI0LCJzdWIiOiJkdW1teS1zdWIiLCJ1aWQiOiJhMTc1YmNlNy0zZTViLTQ4NjMtOTJhMS1lZmMxOTkxYWU2ZmQ6ZWZnaTVlaDVwd2lqIiwiY2xhaW1zIjp7ImJyYW5kSWQiOiJkdW1teS1icmFuZC1pZCIsImVneW1BY2NvdW50SWQiOiJkdW1teS1lZ3ltLWFjY291bnQtaWQiLCJtZW1iZXJzaGlwSWQiOiJkdW1teS1tZW1iZXJzaGlwLWlkIiwibW1zTWVtYmVyc2hpcElkcyI6WyIxMjM0NTY3ODkwIl19fQ.C_NkEF_U8PNPfSSX_P-aYZdssOygvhz3Q8QEGfbEnkI";
    const SESS_ID_DUMMY: &str = "PHPSESSID123DUMMY";
    const EGYM_LOGIN_ERR_DUMMY: &str = "Egym login test-failure";
    const FF_LOGIN_ERR_DUMMY: &str = "FF login test-failure";

    #[tokio::test]
    async fn egym_login_success() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            Some(Err(Box::from("FF-login not tested here"))),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );

        let mut login_service = LoginService::new(http_client_mock, Arc::new(CookieMock));
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_some());
        assert_eq!(EGYM_JWT_DUMMY, login_service.token.unwrap());
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
        let mut login_service = LoginService::new(http_client_mock, Arc::new(CookieMock));
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_none());
    }

    #[tokio::test]
    async fn ff_login_success() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            Some(ff_login_response_dummy(SESS_ID_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let mut login_service = LoginService::new(http_client_mock, Arc::new(CookieMock));
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

        assert!(success.is_ok());
        assert!(login_service.token.is_some());
        assert!(login_service.session.is_some());
        assert_eq!(SESS_ID_DUMMY, login_service.session.unwrap());
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
        let mut login_service = LoginService::new(http_client_mock, Arc::new(CookieMock));
        let req = EgymLoginRequest::new("user", "password");
        let success = login_service.do_login(req).await;

        assert!(success.is_err());
        assert!(login_service.token.is_some());
        assert_eq!(EGYM_JWT_DUMMY, login_service.token.unwrap());
        assert!(login_service.session.is_none());
    }

    #[tokio::test]
    async fn can_decode_user_id_from_token() {
        let token_res_dummy = format!("{EGYM_TOKEN_URL_DUMMY}{EGYM_JWT_DUMMY}");
        let http_client_mock = mock_client!(
            Some(egym_login_response_dummy(&token_res_dummy)),
            Some(ff_login_response_dummy(SESS_ID_DUMMY)),
            MockRes::None,
            MockRes::None,
            MockRes::None
        );
        let mut login_service = LoginService::new(http_client_mock, Arc::new(CookieMock));
        let req = EgymLoginRequest::new("user", "password");
        let _success = login_service.do_login(req).await;

        assert!(login_service.token.is_some());
        assert_eq!(EGYM_JWT_DUMMY, login_service.token.as_ref().unwrap());
        let user_id = login_service
            .get_login_credentials()
            .expect("dummy should have login creds")
            .user_id;
        assert_eq!(user_id, 1234567890);
    }
}
