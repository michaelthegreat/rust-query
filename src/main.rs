use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;
use dotenv::dotenv;
/* get secret example in java\\\\]]]]]]]]]]]]\

// Use this code snippet in your app.
// If you need more information about configurations or implementing the sample
// code, visit the AWS docs:
// https://docs.aws.amazon.com/sdk-for-java/latest/developer-guide/home.html

// Make sure to import the following packages in your code
// import software.amazon.awssdk.regions.Region;
// import software.amazon.awssdk.services.secretsmanager.SecretsManagerClient;
// import software.amazon.awssdk.services.secretsmanager.model.GetSecretValueRequest;
// import software.amazon.awssdk.services.secretsmanager.model.GetSecretValueResponse;	

public static void getSecret() {

    String secretName = "michaels-art-site-db";
    Region region = Region.of("us-east-1");

    // Create a Secrets Manager client
    SecretsManagerClient client = SecretsManagerClient.builder()
            .region(region)
            .build();

    GetSecretValueRequest getSecretValueRequest = GetSecretValueRequest.builder()
            .secretId(secretName)
            .build();

    GetSecretValueResponse getSecretValueResponse;

    try {
        getSecretValueResponse = client.getSecretValue(getSecretValueRequest);
    } catch (Exception e) {
        // For a list of exceptions thrown, see
        // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html
        throw e;
    }

    String secret = getSecretValueResponse.secretString();

    // Your code goes here.
} */


/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // important todo: on a lambda this seems risky to do, but I'm not sure how to get the connection string otherwise.
    dotenv().ok();
    
    let postgres_db_host = env::var("POSTGRES_DB_HOST").expect("Error: Working directory environment variable POSTGRES_DB_HOST not found");
    let postgres_db_port = env::var("POSTGRES_DB_PORT").expect("Error: Working directory environment variable POSTGRES_DB_PORT not found");
    let postgres_db_name = env::var("POSTGRES_DB_NAME").expect("Error: Working directory environment variable POSTGRES_DB_NAME not found");
    let postgres_db_user = env::var("POSTGRES_DB_USER").expect("Error: Working directory environment variable POSTGRES_DB_USER not found");
    let postgres_db_password = env::var("POSTGRES_DB_PASSWORD").expect("Error: Working directory environment variable POSTGRES_DB_PASSWORD not found");
    
    let connection_string = format!("postgres://{}:{}@{}:{}/{}",
    postgres_db_user,
    postgres_db_password,
    postgres_db_host,
    postgres_db_port,
    postgres_db_name);

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
        // .body(message.into())
        .body(connection_string.into())
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
