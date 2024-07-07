use booking_first_lib::fitness_service::FitnessService;
use lambda_common::reqwest_client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let session = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("session"));

    let user_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("user_id"));

    match session {
        Some(session) => {
            let http_client = reqwest_client();

            let fitness_service = FitnessService::new(http_client);

            let courses = fitness_service
                .fetch_courses(session, user_id)
                .await
                .expect("fetching courses");
            let courses = serde_json::to_string(&courses).expect("convert courses into String");

            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(courses.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        None => Err(Box::from("no session no courses")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
