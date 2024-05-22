use std::sync::Arc;

use fitness_api::{args::Args, ui};

use fitness_api::http_client::ReqwestHttpClient;

use fitness_api::http_client::FetchApiClient;

use reqwest::cookie::Jar;
use wasm_bindgen::prelude::*;

#[cfg(not(target_family = "wasm"))]
#[tokio::main(flavor = "current_thread")]
#[wasm_bindgen(start)]
async fn main() -> Result<(), JsValue> {
    //async fn main() -> Result<(), Box<dyn Error>> {
    let args = <Args as clap::Parser>::parse();

    let cookie_jar = Arc::new(Jar::default());

    #[cfg(not(target_family = "wasm"))]
    {
        let client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Could not create client");
        let http_client = ReqwestHttpClient { client };
        ui::run_ui(http_client, cookie_jar, args);
    };

    #[cfg(target_family = "wasm")]
    {
        let client = web_sys::window().unwrap();
        let http_client = FetchApiClient { client };
        ui::run_ui(http_client, cookie_jar, args);
    };

    Ok(())
}
