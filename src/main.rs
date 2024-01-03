use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;
use dotenv::dotenv;
use std::thread;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rust_decimal::prelude::*;
use aws_parameters_and_secrets_lambda::Manager;
use serde::Deserialize;

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

mod connect {
    pub fn do_aws_secrets_manager_connection (secret_name: &str) {
        let manager = super::Manager::default();
        let secret = manager.get_secret(secret_name);
        
        println!("secret: {:?}", secret);
        println!("TODO do_aws_secrets_manager_connection");
    } 

    pub fn do_env_connection () {
        println!("TODO do_env_connection");
    }
}

enum ConnectionType {
    AwsSecretsManager,
    Env,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // important todo: on a lambda this seems risky to do, but I'm not sure how to get the connection string otherwise.
    dotenv().ok();
    // assert_eq!("", secret_value.api_key);
    let connection_type = ConnectionType::AwsSecretsManager;
    match connection_type {
        ConnectionType::AwsSecretsManager => {
            let secret_name = env::var("SECRET_NAME").expect("Error: Working directory environment variable SECRET_NAME not found");
            connect::do_aws_secrets_manager_connection(&secret_name);
        },
        ConnectionType::Env => {
            connect::do_env_connection();
        }
    }


    
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
            json_response.push_str(&format!("{{ \"listingId\": {}, \"title\": \"{}\", \"description\": \"{}\", \"price\": {}, \"inStock\": {}, \"length\": {}, \"width\": {}, \"imageUrl\": \"{}\", \"deleted\": {} }},", listing.listing_id, listing.title, listing.description, listing.price, listing.in_stock, listing.length, listing.width, listing.image_url, listing.deleted));
        }
        json_response.pop();
        json_response.push_str("]}");
        json_response
        
    });


    let response = query_result.join().unwrap();
    // let response = "hello";

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
