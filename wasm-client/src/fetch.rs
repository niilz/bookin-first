use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, Response};

pub(crate) async fn client(
    method: &str,
    url: &str,
    body: Option<&str>,
) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or("Window unavailable")?;
    let mut init = RequestInit::new();
    init.body(Some(&JsValue::from(body)));
    init.method(method);

    let promise = window.fetch_with_str_and_init(url, &init);
    let res = JsFuture::from(promise).await?.dyn_into::<Response>()?;

    JsFuture::from(res.json()?).await
}

// TODO: make configurable
const LAMBDA_BASE_URL: &str = "http://localhost:9000/lambda-url/";

pub(crate) fn lambda_url(func: &str, session: Option<&str>) -> String {
    let query = match session {
        Some(session) => format!("?session={session}"),
        None => "".to_string(),
    };
    format!("{LAMBDA_BASE_URL}{func}{query}")
}

#[cfg(test)]
mod tests {
    // TODO:
}
