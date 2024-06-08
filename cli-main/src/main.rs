use booking_first_lib::dto::error::BoxDynError;
use cli::args::Args;

use booking_first_lib::http_client::reqwest_client::ReqwestHttpClientSend;

mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), BoxDynError> {
    let args = <Args as clap::Parser>::parse();

    let client = reqwest::Client::builder()
        .build()
        .expect("Could not create client");
    let http_client = ReqwestHttpClientSend { client };
    cli::run_cli(http_client, args).await?;
    Ok(())
}
