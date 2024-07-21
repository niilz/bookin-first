use std::collections::HashMap;

use fetch::Window;
use shared::dto::login_data::LoginData;
use shared::dto::request::BookingRequest;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Window as WebSysWindow;

mod fetch;

#[wasm_bindgen]
pub async fn login(user_name: String, password: String, mode: &str) -> Result<JsValue, JsValue> {
    let login_data = LoginData {
        user_name,
        password,
    };
    let login_url = fetch::lambda_url("login-lambda", &HashMap::from([("mode", mode)]));
    let login_data = serde_json::to_string(&login_data).expect("login_data to Json");

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    fetch::client("POST", &login_url, Some(&login_data), &window).await
}

#[wasm_bindgen]
pub async fn courses(session_id: &str, user_id: &str) -> Result<JsValue, JsValue> {
    let mut params = default_params(session_id);
    if !user_id.is_empty() {
        params.insert("user_id", user_id);
    }
    let courses_url = fetch::lambda_url("courses-lambda", &params);

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    fetch::client("GET", &courses_url, None, &window).await
}

#[wasm_bindgen]
pub async fn slots(session_id: &str, course_id: &str) -> Result<JsValue, JsValue> {
    let mut params = default_params(session_id);
    params.insert("course", course_id);
    let slots_url = fetch::lambda_url("slots-lambda", &params);

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    fetch::client("GET", &slots_url, None, &window).await
}

#[wasm_bindgen]
pub async fn book_course(
    booking_request: BookingRequest,
    session_id: &str,
    user_id: &str,
    cancel: bool,
) -> Result<JsValue, JsValue> {
    let mut params = default_params(session_id);
    //dbg!(cancel);
    let cancel = cancel.to_string();
    params.insert("userId", user_id);
    params.insert("cancel", &cancel);

    let booking_url = fetch::lambda_url("book-lambda", &params);

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    let booking_request = serde_json::to_string(&booking_request).expect("booking_req to Json");

    fetch::client("POST", &booking_url, Some(&booking_request), &window).await
}

fn default_params(session_id: &str) -> HashMap<&str, &str> {
    HashMap::from([("session", session_id)])
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
