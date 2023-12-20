use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;
use dotenv::dotenv;
use std::thread;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rust_decimal::prelude::*;
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
struct Listing {
    listing_id: i32,
    title: String,
    description: String,
    price: i64,
    in_stock: i32,
    length: Decimal,
    width: Decimal,
    image_url: String,
    deleted: bool,
}

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

    let manager = PostgresConnectionManager::new(
        connection_string.parse().unwrap(),
        NoTls,
    );
    const QUERY: &str = r#"SELECT "listingId", "title", "description", "price", "inStock", "length", "width", "height", "imageUrl", "deleted", "createdOn" FROM "public"."listing" AS "ListingModel" WHERE "ListingModel"."deleted" = false;"#;
    let pool = r2d2::Pool::new(manager).unwrap();
    
    let query_result =  thread::spawn( move || {
        let mut client = pool.get().unwrap();
        let result = client.query(QUERY, &[]).unwrap();

        let mut json_response = String::new();
        json_response.push_str("{ \"listings\": [");

        for i in 0..result.len() {
            let listing = Listing {
                listing_id: result.get(i).unwrap().get(0),
                title: result.get(i).unwrap().get(1),
                description: result.get(i).unwrap().get(2),
                price: result.get(i).unwrap().get(3),
                in_stock: result.get(i).unwrap().get(4),
                length: result.get(i).unwrap().get(5),
                width: result.get(i).unwrap().get(6),
                image_url: result.get(i).unwrap().get(8),
                deleted: result.get(i).unwrap().get(9),
            };
            json_response.push_str(&format!("{{ \"listing_id\": {}, \"title\": {}, \"description\": {}, \"price\": {}, \"in_stock\": {}, \"length\": {}, \"width\": {}, \"image_url\": {}, \"deleted\": {} }},", listing.listing_id, listing.title, listing.description, listing.price, listing.in_stock, listing.length, listing.width, listing.image_url, listing.deleted));
        }
        json_response.pop();
        json_response.push_str("]}");
        json_response
        
    });

    let response = query_result.join().unwrap();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response.into())
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
