use std::{collections::HashMap, error::Error, sync::Arc};

use crate::{
    dto::request::{EgymLoginRequest, FitnessFirstLoginRequest},
    dto::response::Response,
};

pub const FITNESS_FIRST_BASE_URL: &str = "https://mein.fitnessfirst.de";
const FITNESS_FIRST_CALLBACK_URL: &str = "https://www.fitnessfirst.de/mein-fitnessfirst";
const EGYM_LOGIN_URL: &str = "https://id.egym.com/login";
const EGYM_TOKEN_PATH: &str = "/egymid-login?token=";
const COURSES_URL_PATH: &str = "/api/magicline/openapi/classes/hamburg3";

// TODO: Remove when async fn in traits is fully stable (see: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits)
#[trait_variant::make(HttpClientSend: Send)]
pub trait HttpClient {
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, Box<dyn Error>>;
    async fn ff_login(&self, request: FitnessFirstLoginRequest)
        -> Result<Response, Box<dyn Error>>;
    async fn read_courses(&self, session_id: &str) -> Result<Response, Box<dyn Error>>;
    async fn read_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, Box<dyn Error>>;
}

pub struct ReqwestHttpClient {
    pub client: reqwest::Client,
}

impl HttpClient for ReqwestHttpClient {
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("username", request.user_name.as_str());
        params.insert("password", request.password.as_str());
        params.insert("clientId", request.client_id);
        params.insert("callbackUrl", FITNESS_FIRST_CALLBACK_URL);
        let res = self.client.post(EGYM_LOGIN_URL).form(&params).send().await;
        match res {
            Ok(res) => {
                let res = res.text().await.expect("could not read response text");
                Ok(Response::Text(res))
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn ff_login(
        &self,
        request: FitnessFirstLoginRequest,
    ) -> Result<Response, Box<dyn Error>> {
        //https://mein.fitnessfirst.de/egymid-login?token=
        let url = format!(
            "{FITNESS_FIRST_BASE_URL}{EGYM_TOKEN_PATH}{}",
            request.egym_token
        );
        println!("Logging in to: {url}");
        let res = self.client.get(url).send().await;
        match res {
            Ok(_res) => Ok(Response::SessionSet),
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn read_courses(&self, session_id: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}");
        println!("Getting courses from: {url}");
        let res = self
            .client
            .get(url)
            .header("Cookie", session_id)
            .send()
            .await;
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read courses: {e}"))),
        }
    }

    async fn read_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, Box<dyn Error>> {
        let courses_url = format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}");
        let slots_url = format!("{courses_url}/{course_id}/slots");
        println!("Getting slots from: {slots_url}");
        let res = self
            .client
            .get(slots_url)
            .header("Cookie", session_id)
            .send()
            .await;
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read slots: {e}"))),
        }
    }
}

impl<Client> HttpClient for Arc<Client>
where
    Client: HttpClient,
{
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, Box<dyn Error>> {
        self.as_ref().egym_login(request).await
    }

    async fn ff_login(
        &self,
        request: FitnessFirstLoginRequest,
    ) -> Result<Response, Box<dyn Error>> {
        self.as_ref().ff_login(request).await
    }

    async fn read_courses(&self, session_id: &str) -> Result<Response, Box<dyn Error>> {
        self.as_ref().read_courses(session_id).await
    }

    async fn read_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, Box<dyn Error>> {
        self.as_ref().read_slots(course_id, session_id).await
    }
}
