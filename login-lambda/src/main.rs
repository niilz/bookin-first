use std::sync::Arc;

use booking_first_lib::{
    booking_service::BookingService, dto::login_data::LoginData,
    http_client::reqwest_client::ReqwestHttpClient,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use reqwest::cookie::Jar;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let Body::Text(request_body) = event.into_body() else {
        return Err(Box::from("Only Text Requests are supported"));
    };

    //dbg!(&request_body);

    match serde_json::from_str(&request_body) {
        Ok(LoginData {
            user_name,
            password,
        }) => {
            let cookie_jar = Arc::new(Jar::default());
            let client = reqwest::Client::builder()
                .cookie_provider(Arc::clone(&cookie_jar))
                .build()
                .expect("Could not create client");
            let http_client = ReqwestHttpClient { client };

            let http_client = Arc::new(http_client);

            let mut booking_service = BookingService::new(http_client, cookie_jar);

            let login_credentials = booking_service
                .login(&user_name, &password)
                .await
                .expect("LoginCreds not present after login?");

            println!("{login_credentials:?}");
        }
        Err(e) => {
            eprintln!("{e:?}");
            return Err(Box::from("Could not parse LoginData"));
        }
    };

    // Extract some useful information from the request
    /*
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    */
    let message = format!("Hello {{who}}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
