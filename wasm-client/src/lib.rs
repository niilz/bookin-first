use shared::dto::login_data::LoginData;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, Response};

#[wasm_bindgen]
pub async fn login(user_name: String, password: String) -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("Window unavailable");
    let login_data = LoginData {
        user_name,
        password,
    };
    let login_data = serde_json::to_string(&login_data).expect("login_data to Json");
    let mut init = RequestInit::new();
    init.body(Some(&JsValue::from(login_data)));
    init.method("POST");

    let promise = window.fetch_with_str_and_init(&lambda_url("login-lambda", None), &init);
    let res = JsFuture::from(promise)
        .await?
        .dyn_into::<Response>()
        .expect("fetch response");

    let login_creds = JsFuture::from(res.json()?).await?;

    Ok(login_creds)
}

const LAMBDA_BASE_URL: &str = "http://localhost:9000/lambda-url/";

fn lambda_url(func: &str, session: Option<&str>) -> String {
    let query = match session {
        Some(session) => format!("?session={session}"),
        None => "".to_string(),
    };
    format!("{LAMBDA_BASE_URL}{func}{query}")
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
