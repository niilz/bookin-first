use std::{collections::HashMap, error::Error};

use crate::{request::EgymLoginRequest, response::EgymLoginResponse};

const MEIN_FITNESS_FIRST_URL: &str = "https://www.fitnessfirst.de/mein-fitnessfirst";
const EGYM_LOGIN_URL: &str = "https://id.egym.com/login";
//const EGYM_LOGIN_URL: &str = "https://httpbin.org/post";

pub trait HttpClient {
    async fn egym_login(
        &self,
        request: EgymLoginRequest,
    ) -> Result<EgymLoginResponse, Box<dyn Error>>;
    async fn ff_login(
        &self,
        request: FitnessFirstLoginRequest,
    ) -> Result<FitnessFirstLoginResponse, Box<dyn Error>>;
}

pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl HttpClient for ReqwestHttpClient {
    async fn egym_login(
        &self,
        request: EgymLoginRequest,
    ) -> Result<EgymLoginResponse, Box<dyn Error>> {
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
                    EgymLoginResponse {
                        egym_jwt: "Dummy".to_string(),
                    }
                })
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }
}
