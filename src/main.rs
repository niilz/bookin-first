use std::error::Error;
use std::sync::Arc;

use cli::args::Args;

use fitness_api::http_client::reqwest_client::ReqwestHttpClient;
use reqwest::cookie::Jar;

mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = <Args as clap::Parser>::parse();

    let cookie_jar = Arc::new(Jar::default());

    #[cfg(not(target_family = "wasm"))]
    {
        let client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Could not create client");
        let http_client = ReqwestHttpClient { client };
        cli::run_cli(http_client, cookie_jar, args).await?;
    };
    Ok(())
}
