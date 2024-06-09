use booking_first_lib::{booking_service::BookingService, dto::login_data::LoginData};
use lambda_common::reqwest_client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let Body::Text(request_body) = event.into_body() else {
        return Err(Box::from("Only Text Requests are supported"));
    };

    match serde_json::from_str(&request_body) {
        Ok(LoginData {
            user_name,
            password,
        }) => {
            let booking_service = BookingService::new(reqwest_client());

            let login_credentials = booking_service
                .login(&user_name, &password)
                .await
                .expect("LoginCreds not present after login?");
            println!("{login_credentials:?}");

            let login_creds_value =
                serde_json::to_string(&login_credentials).expect("Convert LoginCreds to String");

            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(login_creds_value.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        Err(e) => {
            eprintln!("{e:?}");
            Err(Box::from("Could not parse LoginData"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
