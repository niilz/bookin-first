use std::collections::HashMap;

use crate::{request::LoginRequest, response::LoginResponse};

const MEIN_FITNESS_FIRST_URL: &str = "https://www.fitnessfirst.de/mein-fitnessfirst";
const EGYM_LOGIN_URL: &str = "https://id.egym.com/login";
//const EGYM_LOGIN_URL: &str = "https://httpbin.org/post";

pub trait HttpClient {
    async fn do_login(&self, request: LoginRequest) -> Result<LoginResponse, ()>;
}

pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl HttpClient for ReqwestHttpClient {
    async fn do_login(&self, request: LoginRequest) -> Result<LoginResponse, ()> {
        let mut params = HashMap::new();
        params.insert("username", request.user_name.as_str());
        params.insert("password", request.password.as_str());
        params.insert("clientId", request.client_id.as_str());
        params.insert("callbackUrl", MEIN_FITNESS_FIRST_URL);
        let res = self.client.post(EGYM_LOGIN_URL).form(&params).send().await;
        match res {
            Ok(res) => {
                println!(
                    "Response: {:#?}",
                    res.text().await.expect("could not read response text")
                );
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
