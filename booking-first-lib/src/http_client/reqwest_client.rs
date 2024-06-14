use std::collections::HashMap;

use shared::dto::{request::EgymLoginRequest, response::Response};

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
                let res = res.text().await.expect("could not read response text");
                Ok(Response::Text(res))
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
                let cookies = res
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
                    .expect("No cookies");
                let session = extract_session(&cookies)?;
                Ok(Response::Session(session))
            }
            Err(e) => Err(Box::from(format!("Failed to login: {e}"))),
        }
    }

    async fn fetch_courses(&self, session_id: &str) -> Result<Response, BoxDynError> {
        let url = format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}");
        println!("Getting courses from: {url}");
        let req = self
            .client
            .get(url)
            .header("Cookie", &format!("PHPSESSID={session_id}"));
        //dbg!(&req);
        let res = req.send().await;
        //dbg!(&res);
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
    ) -> Result<Response, BoxDynError> {
        //https://mein.fitnessfirst.de/api/magicline/openapi/classes/hamburg3/booking/book
        let booking_url = format!("{FITNESS_FIRST_BASE_URL}{COURSES_URL_PATH}/booking/book");
        let booking = serde_json::to_string(&booking)?;
        //dbg!(&booking);
        let req = self
            .client
            .post(booking_url)
            .body(booking)
            .header("Cookie", &format!("PHPSESSID={session_id}"));
        //dbg!(&req);
        let res = req.send().await;
        //dbg!(&res);
        match res {
            Ok(res) => Ok(Response::Json(res.text().await?)),
            Err(e) => Err(Box::from(format!("Failed to read slots: {e}"))),
        }
    }
}

fn extract_session(cookies: &str) -> Result<String, String> {
    let session = cookies
        .split("; ")
        .filter(|cookie| cookie.starts_with("PHPSESSID="))
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
        let session = extract_session(&dummy_cookies);
        assert_eq!(session.unwrap(), expected_session);
    }

    #[test]
    fn no_session_is_error() {
        let dummy_cookies_without_session = format!("path=/; secure; httponly; samesite=lax");
        let no_session = extract_session(&dummy_cookies_without_session);
        assert!(no_session.is_err());
    }
}
