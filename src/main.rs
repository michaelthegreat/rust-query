use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
// use std::env;
// use dotenv::dotenv;
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // important todo: on a lambda this seems risky to do, but I'm not sure how to get the connection string otherwise.
    // let POSTGRES_DB_HOST = env::var("POSTGRES_DB_HOST").expect("Error: Working directory environment variable POSTGRES_DB_HOST not found");
    // let POSTGRES_DB_PORT = env::var("POSTGRES_DB_PORT").expect("Error: Working directory environment variable POSTGRES_DB_PORT not found");
    // let POSTGRES_DB_NAME = env::var("POSTGRES_DB_NAME").expect("Error: Working directory environment variable POSTGRES_DB_NAME not found");
    // let POSTGRES_DB_USER = env::var("POSTGRES_DB_USER").expect("Error: Working directory environment variable POSTGRES_DB_USER not found");
    // let POSTGRES_DB_PASSWORD = env::var("POSTGRES_DB_PASSWORD").expect("Error: Working directory environment variable POSTGRES_DB_PASSWORD not found");
    
    // let connection_string = format!("postgres://{}:{}@{}:{}/{}",
    // POSTGRES_DB_USER,
    // POSTGRES_DB_PASSWORD,
    // POSTGRES_DB_HOST,
    // POSTGRES_DB_PORT,
    // POSTGRES_DB_NAME);

    // println!("{}", connection_string);
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        // .body(connection_string.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
