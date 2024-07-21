use std::str::FromStr;

use booking_first_lib::fitness_service::FitnessService;
use lambda_common::reqwest_client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use shared::dto::request::BookingRequest;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let Body::Text(booking) = event.body() else {
        return Err(Box::from("Only Text Request is supported"));
    };

    //dbg!(&booking);

    let session = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("session"));

    let user_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("userId"));

    let cancel = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("cancel"))
        .unwrap_or("false");
    let cancel = bool::from_str(cancel).expect("cancesl should be true or false");

    match (session, serde_json::from_str::<BookingRequest>(booking)) {
        (Some(session), Ok(booking_request)) => {
            let http_client = reqwest_client();

            let fitness_service = FitnessService::new(http_client);

            let slot = fitness_service
                .book_course(booking_request, session, user_id, cancel)
                .await
                .expect("booking course");

            let booking = serde_json::to_string(&slot).expect("convert booking into String");

            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(booking.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        _ => Err(Box::from("booking-data and session required")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
