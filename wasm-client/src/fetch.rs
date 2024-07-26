use std::collections::HashMap;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, RequestInit, RequestMode, Response, Window as WebSysWindow};

pub(crate) trait Window {
    fn instance() -> Option<impl Window>;
    fn fetch(&self, url: &str, init: &RequestInit) -> js_sys::Promise;
}

impl Window for WebSysWindow {
    fn instance() -> Option<impl Window> {
        web_sys::window()
    }
    fn fetch(&self, url: &str, init: &RequestInit) -> js_sys::Promise {
        self.fetch_with_str_and_init(url, init)
    }
}

pub(crate) async fn client(
    method: &str,
    url: &str,
    body: Option<&str>,
    window: &impl Window,
) -> Result<JsValue, JsValue> {
    let mut init = RequestInit::new();
    init.body(Some(&JsValue::from(body)));
    init.method(method);
    init.mode(RequestMode::Cors);

    let promise = window.fetch(url, &init);
    let future = JsFuture::from(promise).await?;
    //console_log!("future: {future:?}");
    let res = future.dyn_into::<Response>()?;
    //console_log!("res: {res:?}");
    let json = res.json()?;
    //console_log!("json: {json:?}");
    let json_future = JsFuture::from(json).await;
    //console_log!("future: {future:?}");
    json_future
}

// TODO: make configurable
const LAMBDA_BASE_URL: &str = "lambda-url.eu-central-1.on.aws/";

pub(crate) fn lambda_url(func: &str, params: &HashMap<&str, &str>) -> String {
    let query = if !params.is_empty() {
        let params = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        format!("?{params}")
    } else {
        "".to_string()
    };
    format!("https://{func}.{LAMBDA_BASE_URL}{query}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::{
        js_sys::{self, JSON},
        RequestInit, Response,
    };

    use crate::fetch::client;

    use super::{lambda_url, Window};

    const SESSION_ID_DUMMY: &str = "12345";
    const LAMBDA_NAME_DUMMY: &str = "frobnify";
    const COURSE_ID_DUMMY: &str = "45678";

    #[test]
    #[wasm_bindgen_test]
    fn url_with_session() {
        let url = lambda_url(
            LAMBDA_NAME_DUMMY,
            &HashMap::from([("session", SESSION_ID_DUMMY)]),
        );
        assert_eq!(
            url,
            "http://localhost:9000/lambda-url/frobnify?session=12345"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn url_with_session_and_course() {
        let url = lambda_url(
            LAMBDA_NAME_DUMMY,
            &HashMap::from([("session", SESSION_ID_DUMMY), ("course", COURSE_ID_DUMMY)]),
        );
        assert!(url.starts_with("http://localhost:9000/lambda-url/"));
        // Since params is a hasmap the order could be differernt
        assert!(
            url.ends_with("frobnify?session=12345&course=45678")
                || url.ends_with("frobnify?course=45678&session=12345")
        );
    }

    struct MockWindow;

    const JSON_DUMMY: &str = r#"{"key":"value"}"#;

    impl Window for MockWindow {
        fn instance() -> Option<impl Window> {
            Some(MockWindow)
        }
        fn fetch(&self, _url: &str, _init: &RequestInit) -> js_sys::Promise {
            let response = Response::new_with_opt_str(Some(JSON_DUMMY)).unwrap();
            js_sys::Promise::resolve(&JsValue::from(&response))
        }
    }

    #[wasm_bindgen_test]
    async fn login_by_user_and_pasword() {
        let window = MockWindow::instance().unwrap();

        let json_res = client("POST", "foo", Some("body"), &window).await;

        assert!(json_res.is_ok());

        assert_eq!(
            JSON::stringify(&json_res.unwrap()).unwrap().to_string(),
            JSON_DUMMY
        );
    }
}
