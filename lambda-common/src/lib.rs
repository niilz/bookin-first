use std::sync::Arc;

use reqwest::cookie::Jar;

pub fn reqwest_client() -> impl HttpClient {
    let cookie_jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_provider(Arc::clone(&cookie_jar))
        .build()
        .expect("Could not create client");
    let http_client = ReqwestHttpClient { client };

    Arc::new(http_client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_client() {
        let client = reqwest_client();
    }
}
