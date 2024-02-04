#![allow(clippy::result_large_err)]
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::thread;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rust_decimal::prelude::*;
mod connect;

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
    let connection = connect::getConnection().await;
    let result = match connection {
        Ok(connection) => {
            let connection_string = format!("postgres://{}:{}@{}:{}/{}",
            connection.username,
            connection.password,
            connection.host,
            connection.port,
            connection.database);
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
                println!("json response {:?}", json_response);

                json_response
                
            });
            query_result
        },
        Err(e) => {
            // return the formatted error
            panic!("Error: {:?}", e);
        }
    };


    let response = result.join().unwrap();

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
