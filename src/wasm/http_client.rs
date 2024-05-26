use std::{collections::HashMap, error::Error};

use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::JsFuture;
use web_sys::{FormData, Headers, Request, RequestInit, RequestMode};

use crate::{
    dto::{
        request::{BookingRequest, EgymLoginRequest, FitnessFirstLoginRequest},
        response::Response,
    },
    http_client::*,
};

#[wasm_bindgen]
pub struct FetchApiClient {
    client: web_sys::Window,
}

#[wasm_bindgen]
impl FetchApiClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            client: web_sys::window().expect("no window object?"),
        }
    }
    pub async fn call(&self) -> JsValue {
        JsFuture::from(self.client.fetch_with_str("https://google.com"))
            .await
            .expect("did not fetch")
    }
}

impl HttpClient for FetchApiClient {
    async fn egym_login(&self, request: EgymLoginRequest) -> Result<Response, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("username", request.user_name.as_str());
        params.insert("password", request.password.as_str());
        params.insert("clientId", request.client_id);
        params.insert("callbackUrl", FITNESS_FIRST_CALLBACK_URL);
        //let res = self.client.post(EGYM_LOGIN_URL).form(&params).send().await;

        let request_params = FormData::new().expect("Creating a FormData object should work");
        let _ = request_params.append_with_str("username", request.user_name.as_str());
        let _ = request_params.append_with_str("password", request.password.as_str());
        let _ = request_params.append_with_str("clientId", request.client_id);
        let _ = request_params.append_with_str("callbackUrl", FITNESS_FIRST_CALLBACK_URL);

        let mut request_options = RequestInit::new();
        request_options.method("POST");
        request_options.body(Some(&request_params));
        request_options.mode(RequestMode::Cors);

        let headers = Headers::new().expect("Create Headers");
        let _ = headers.append(
            "content-type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        );
        request_options.headers(&headers);

        let request = Request::new_with_str_and_init(EGYM_LOGIN_URL, &request_options)
            .expect("Create Egym-Login-Request");

        let res = JsFuture::from(self.client.fetch_with_request(&request))
            .await
            .expect("did not fetch");

        web_sys::console::log_1(&format!("{res:?}").into());

        println!("{res:?}");
        Ok(Response::Text("foo".to_string()))
    }

    async fn ff_login(
        &self,
        request: FitnessFirstLoginRequest,
    ) -> Result<Response, Box<dyn Error>> {
        todo!()
    }

    async fn fetch_courses(&self, session_id: &str) -> Result<Response, Box<dyn Error>> {
        todo!()
    }

    async fn fetch_slots(
        &self,
        course_id: usize,
        session_id: &str,
    ) -> Result<Response, Box<dyn Error>> {
        todo!()
    }

    async fn book_course(
        &self,
        booking: BookingRequest,
        session_id: &str,
    ) -> Result<Response, Box<dyn Error>> {
        todo!()
    }
}
