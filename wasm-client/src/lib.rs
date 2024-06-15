use fetch::Window;
use shared::dto::login_data::LoginData;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Window as WebSysWindow;

mod fetch;

#[wasm_bindgen]
pub async fn login(user_name: String, password: String) -> Result<JsValue, JsValue> {
    let login_data = LoginData {
        user_name,
        password,
    };
    let login_url = fetch::lambda_url("login-lambda", None);
    let login_data = serde_json::to_string(&login_data).expect("login_data to Json");

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    fetch::client("POST", &login_url, Some(&login_data), &window).await
}

#[wasm_bindgen]
pub async fn fetch_courses(session_id: &str) -> Result<JsValue, JsValue> {
    let courses_url = fetch::lambda_url("courses-lambda", Some(session_id));

    let window = WebSysWindow::instance().ok_or("Window unavailable")?;

    fetch::client("GET", &courses_url, None, &window).await
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
