use booking_first_lib::fitness_service::FitnessService;
use lambda_common::reqwest_client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let course_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("course"))
        .and_then(|session| session.parse::<usize>().ok());

    let session = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("session"));

    match (course_id, session) {
        (Some(course_id), Some(session)) => {
            let http_client = reqwest_client();

            let fitness_service = FitnessService::new(http_client);

            let slots = fitness_service
                .fetch_slots(course_id, session)
                .await
                .expect("fetching slots");

            let slots = serde_json::to_string(&slots).expect("convert slot into String");

            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(slots.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        (_, _) => Err(Box::from("course_id and session required")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
