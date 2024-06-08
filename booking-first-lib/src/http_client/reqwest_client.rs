use std::{collections::HashMap, error::Error};

use crate::dto::{
    request::{EgymLoginRequest, FitnessFirstLoginRequest},
    response::Response,
};

use super::*;

pub struct ReqwestHttpClientSend {
    pub client: reqwest::Client,
}

impl HttpClientSend for ReqwestHttpClientSend {
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, BoxDynError> {
        let mut params = HashMap::new();
        params.insert("username", request.user_name.as_str());
        params.insert("password", request.password.as_str());
        params.insert("clientId", request.client_id);
        params.insert("callbackUrl", FITNESS_FIRST_CALLBACK_URL);
        let res = self.client.post(EGYM_LOGIN_URL).form(&params).send().await;
        match res {
            Ok(res) => {
                println!(
                    "Cookies from egym_login: {:?}",
                    res.cookies().collect::<Vec<_>>()
                );
                let res = res.text().await.expect("could not read response text");
                Ok(Response::Text(res))
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn ff_login(&self, request: FitnessFirstLoginRequest) -> Result<Response, BoxDynError> {
        //https://mein.fitnessfirst.de/egymid-login?token=
        let url = format!(
            "{FITNESS_FIRST_BASE_URL}{EGYM_TOKEN_PATH}{}",
            request.egym_token
        );
        println!("Logging in to: {url}");
        let res = self.client.get(url).send().await;
        match res {
            Ok(res) => {
                let cookies = res
                    .cookies()
                    .map(|c| c.name().to_string())
                    .collect::<String>();
                println!("Cookies from ff_login: {cookies}",);
                Ok(Response::Cookies(cookies))
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn fetch_courses(&self, session_id: &str) -> Result<Response, BoxDynError> {
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

    async fn fetch_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, BoxDynError> {
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

    async fn book_course(
        &self,
        booking: BookingRequest,
        session_id: &str,
    ) -> Result<Response, BoxDynError> {
        //https://mein.fitnessfirst.de/api/magicline/openapi/classes/hamburg3/booking/book
        let booking_url = format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}/booking/book");
        let booking = serde_json::to_string(&booking)?;
        let res = self
            .client
            .post(booking_url)
            .body(booking)
            .header("Cookie", session_id)
            .send()
            .await;
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read slots: {e}"))),
        }
    }
}
