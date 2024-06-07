use std::sync::Arc;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let Body::Text(request_body) = event.into_body() else {
        return Err(Box::from("Only Text Requests are supported"));
    };

    let http_client = reqwest_client();


    match serde_json::from_str(&request_body) {
        // Extract session-token from request
        // Query fitness-first api for courses
        // Return list of courses
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
