use std::{error::Error, sync::Arc};

use wasm_bindgen::prelude::*;

use crate::{
    cookies::Cookie,
    dto::{
        course::Course,
        request::{BookingRequest, EgymLoginRequest},
        response::BookingResponse,
        slots::Slot,
    },
    fitness_service::FitnessService,
    login_service::{LoginCreds, LoginService},
    wasm::http_client::FetchApiClient,
};

#[wasm_bindgen]
pub struct BookingServiceWasm {
    login_service: LoginServiceWasm,
    fitness_service: FitnessServiceWasm,
}

#[wasm_bindgen]
impl BookingServiceWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(http_client: FetchApiClient, cookie_jar: CookieWasm) -> Self {
        web_sys::console::log_1(&"> new booking service".into());

        let http_client = Arc::new(http_client);
        let cookie_jar = Arc::new(cookie_jar);

        web_sys::console::log_1(&"arced the args".into());

        let login_service = LoginService::new(Arc::clone(&http_client), Arc::clone(&cookie_jar));
        let login_service = LoginServiceWasm(login_service);

        let fitness_service = FitnessService::new(Arc::clone(&http_client));
        let fitness_service = FitnessServiceWasm(fitness_service);

        Self {
            login_service,
            fitness_service,
        }
    }

    pub async fn login(&mut self, user_name: &str, password: &str) -> Result<LoginCreds, JsValue> {
        let login_request = EgymLoginRequest::new(user_name, password);

        let _response = self.login_service.do_login(login_request).await;

        self.login_service
            .get_login_credentials()
            .map_err(|e| JsValue::from_str(&format!("{e:?}")))
    }

    pub async fn fetch_courses(&self, credentials: &LoginCreds) -> JsValue {
        let courses = self
            .fitness_service
            .fetch_courses(credentials)
            .await
            .expect("read courses");

        serde_wasm_bindgen::to_value(&courses).expect("Vec<Course> to JsValue")
    }

    pub async fn fetch_slots(&self, course: &Course, credentials: &LoginCreds) -> JsValue {
        let slots = self
            .fitness_service
            .fetch_slots(course.id, credentials)
            .await
            .expect("read slots");

        serde_wasm_bindgen::to_value(&slots).expect("Vec<Slot> to JsValue")
    }

    pub async fn book_course(
        &self,
        booking: BookingRequest,
        credentials: LoginCreds,
    ) -> Result<BookingResponse, JsValue> {
        self.fitness_service
            .book_course(booking, credentials)
            .await
            .map_err(|e| JsValue::from_str(&format!("{e:?}")))
    }
}

#[wasm_bindgen]
pub struct CookieWasm {
    cookies: Vec<String>,
}

#[wasm_bindgen]
impl CookieWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cookies: Vec::new(),
        }
    }
}

impl Cookie for CookieWasm {
    fn read_cookie(&self, _domain: &str) -> Result<String, Box<dyn Error>> {
        Ok("TODO".to_string())
    }
}

#[wasm_bindgen]
pub struct LoginServiceWasm(LoginService<Arc<FetchApiClient>, Arc<CookieWasm>>);
impl LoginServiceWasm {
    async fn do_login(&mut self, request: EgymLoginRequest) -> Result<(), Box<dyn Error>> {
        self.0.do_login(request).await
    }

    pub fn get_login_credentials(&self) -> Result<LoginCreds, Box<dyn Error>> {
        self.0.get_login_credentials()
    }
}

#[wasm_bindgen]
pub struct FitnessServiceWasm(FitnessService<Arc<FetchApiClient>>);

impl FitnessServiceWasm {
    async fn fetch_courses(&self, credentials: &LoginCreds) -> Result<Vec<Course>, Box<dyn Error>> {
        self.0.fetch_courses(credentials).await
    }

    async fn fetch_slots(
        &self,
        course_id: usize,
        credentials: &LoginCreds,
    ) -> Result<Vec<Slot>, Box<dyn Error>> {
        self.0.fetch_slots(course_id, credentials).await
    }

    async fn book_course(
        &self,
        booking: BookingRequest,
        credentials: LoginCreds,
    ) -> Result<BookingResponse, Box<dyn Error>> {
        self.0.book_course(booking, credentials).await
    }
}
