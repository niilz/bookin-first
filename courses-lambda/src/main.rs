use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let session = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("session"));

    match session {
        Some(session) => {
            // fetchcourses
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(session.into())
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
