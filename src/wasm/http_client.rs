use std::error::Error;

use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::JsFuture;

use crate::{
    dto::{
        request::{BookingRequest, EgymLoginRequest, FitnessFirstLoginRequest},
        response::Response,
    },
    http_client::HttpClient,
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
        let res = JsFuture::from(self.client.fetch_with_str("google.com"))
            .await
            .expect("did not fetch");
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
