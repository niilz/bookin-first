use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use shared::dto::response::Response;

use super::*;

const DAY_SEC: u64 = 60 * 60 * 24;

pub struct ReqwestHttpClientSend {
    pub client: reqwest::Client,
}

impl HttpClientSend for ReqwestHttpClientSend {
    async fn egym_login(&self, request: LoginRequest) -> Result<Response, BoxDynError> {
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

    async fn netpulse_login(&self, request: LoginRequest) -> Result<Response, BoxDynError> {
        let mut params = HashMap::new();
        params.insert("username", request.user_name.as_str());
        params.insert("password", request.password.as_str());
        let req = self.client.post(NETPULSE_LOGIN_URL).form(&params);
        dbg!(&req);
        let response = req.send().await;
        match response {
            Ok(res) => {
                let cookies = read_cookies(&res);
                let session = extract_session(&cookies, "JSESSIONID=")?;
                let response = res.text().await.expect("could not read reponse text");
                Ok(Response::WithSession { response, session })
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn ff_login(&self, egym_token: &str) -> Result<Response, BoxDynError> {
        //https://mein.fitnessfirst.de/egymid-login?token=
        let url = format!("{FITNESS_FIRST_BASE_URL}{EGYM_TOKEN_PATH}{egym_token}");
        println!("Logging in to: {url}");
        let req = self.client.get(url);
        let res = req.send().await;
        match res {
            Ok(res) => {
                //dbg!(&res);
                let cookies = read_cookies(&res);
                let session = extract_session(&cookies, "PHPSESSID=")?;
                Ok(Response::Session(session))
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn fetch_courses(
        &self,
        session_id: &str,
        user_id: Option<&str>,
    ) -> Result<Response, BoxDynError> {
        let url = match user_id {
            None => format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}"),
            Some(uuid) => {
                let start_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("System Time before UNIX-Epoch");
                let two_weeks = Duration::from_secs(DAY_SEC * 14);
                let end_time = start_time
                    .checked_add(two_weeks)
                    .expect("reached end of time")
                    .as_millis();
                let start_time = start_time.as_millis();
                // TODO: make gymlocation variable
                format!("{FF_NETPULSE_BASE_URL}/{GYM_EPPENDORF_LOCATION_ID}/classes?startDateTime={start_time}&exerciserUuid={uuid}&endDateTime={end_time}")
            }
        };
        println!("Getting courses from: {url}");
        let session_id_key = match user_id {
            None => "PHPSESSID",
            Some(_) => "JSESSIONID",
        };
        let req = self
            .client
            .get(url)
            .header("Cookie", &format!("{session_id_key}={session_id}"));
        //dbg!(&req);
        let res = req.send().await;
        //dbg!(&res);
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read courses: {e}"))),
        }
    }

    // Only for WEB-mode
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
            .header("Cookie", &format!("PHPSESSID={session_id}"))
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
        user_id: Option<&str>,
    ) -> Result<Response, BoxDynError> {
        let booking_url = match user_id {
            // https://mein.fitnessfirst.de/api/magicline/openapi/classes/hamburg3/booking/book
            None => format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}/booking/book"),
            // https://fitnessfirst.netpulse.com/np/company/<location_id>/class/<class_id>:<slot_id>/addExerciser
            Some(user_id) => format!(
                "{FF_NETPULSE_BASE_URL}/{GYM_EPPENDORF_LOCATION_ID}/class/{}:{}/addExerciser",
                booking.course_id, booking.slot_id
            ),
        };

        let res = match user_id {
            None => {
                let booking = serde_json::to_string(&booking)?;
                //dbg!(&booking);
                let req = self
                    .client
                    .post(booking_url)
                    .body(booking)
                    .header("Cookie", &format!("PHPSESSID={session_id}"));
                req.send().await
            }
            Some(user_id) => {
                let exerciser_param = HashMap::from([("exerciserUuid", user_id)]);
                let req = self
                    .client
                    .post(booking_url)
                    .form(&exerciser_param)
                    .header(
                        "X-NP-User-Agent",
                        "clientType=MOBILE_DEVICE; devicePlatform=ANDROID",
                    )
                    .header("Cookie", &format!("JSESSIONID={session_id}"));
                dbg!(&req);
                req.send().await
            }
        };
        dbg!(&res);
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read slots: {e}"))),
        }
    }
}

fn read_cookies(response: &reqwest::Response) -> String {
    response
        .headers()
        .iter()
        .filter(|h| h.0 == "set-cookie")
        // TODO: propagate error up
        .map(|h| {
            h.1.to_str()
                .expect("could not convert header to str")
                .to_string()
        })
        // TODO: turn option into error and bubble up
        .last()
        .expect("No cookies")
}

fn extract_session(cookies: &str, session_key: &str) -> Result<String, String> {
    let session = cookies
        .split("; ")
        .filter(|cookie| cookie.starts_with(session_key))
        .filter_map(|cookie| cookie.split_once('='))
        .map(|(_, session)| session)
        .last()
        .ok_or("No Session present")?;

    Ok(session.to_string())
}

#[cfg(test)]
mod tests {
    use crate::http_client::reqwest_client::extract_session;

    #[test]
    fn extract_session_from_cookies() {
        let expected_session = "12345";
        let dummy_cookies =
            format!("PHPSESSID={expected_session}; path=/; secure; httponly; samesite=lax");
        let session = extract_session(&dummy_cookies, "PHPSESSID=");
        assert_eq!(session.unwrap(), expected_session);
    }

    #[test]
    fn no_session_is_error() {
        let dummy_cookies_without_session = format!("path=/; secure; httponly; samesite=lax");
        let no_session = extract_session(&dummy_cookies_without_session, "PHPSESSID=");
        assert!(no_session.is_err());
    }
}
