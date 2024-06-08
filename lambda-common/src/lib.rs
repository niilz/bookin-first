use std::sync::Arc;

use booking_first_lib::http_client::{reqwest_client::ReqwestHttpClientSend, HttpClientSend};
use reqwest::redirect::Policy;

pub fn reqwest_client() -> impl HttpClientSend {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .expect("Could not create client");
    let http_client = ReqwestHttpClientSend { client };

    Arc::new(http_client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_client() {
        let _client = reqwest_client();
    }
}
