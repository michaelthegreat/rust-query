// #[macro_use] extern crate juniper;

// use juniper::{FieldResult};
use std::thread;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2::Pool;
use std::env;
use dotenv::dotenv;
use std::format;

// #[derive(GraphQLEnum)]
// enum Episode {
//     NewHope,
//     Empire,
//     Jedi,
// }

// #[derive(GraphQLObject)]
// #[graphql(description="A humanoid creature in the Star Wars universe")]
// struct Human {
//     id: String,
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

// // There is also a custom derive for mapping GraphQL input objects.

// #[derive(GraphQLInputObject)]
// #[graphql(description="A humanoid creature in the Star Wars universe")]
// struct NewHuman {
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

// // Now, we create our root Query and Mutation types with resolvers by using the
// // graphql_object! macro.
// // Objects can have contexts that allow accessing shared state like a database
// // pool.

// struct Context {
//     // TODO: Use your real database pool here.
//     pool: DatabasePool,
// }

// // To make our context usable by Juniper, we have to implement a marker trait.
// impl juniper::Context for Context {}

// struct Query;

// graphql_object!(Query: Context |&self| {

//     field apiVersion() -> &str {
//         "1.0"
//     }

//     // Arguments to resolvers can either be simple types or input objects.
//     // The executor is a special (optional) argument that allows accessing the context.
//     field human(&executor, id: String) -> FieldResult<Human> {
//         // Get the context from the executor.
//         let context = executor.context();
//         // Get a db connection.
//         let connection = context.pool.get_connection()?;
//         // Execute a db query.
//         // Note the use of `?` to propagate errors.
//         let human = connection.find_human(&id)?;
//         // Return the result.
//         Ok(human)
//     }
// });

// struct Mutation;

// graphql_object!(Mutation: Context |&self| {

//     field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
//         let db = executor.context().pool.get_connection()?;
//         let human: Human = db.insert_human(&new_human)?;
//         Ok(human)
//     }
// });

// // A root schema consists of a query and a mutation.
// // Request queries can be executed against a RootNode.
// type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() {
    // Retrieve the value of the "PWD" environment variable
    dotenv().ok();
    let POSTGRES_DB_HOST = env::var("POSTGRES_DB_HOST").expect("Error: Working directory environment variable POSTGRES_DB_HOST not found");
    let POSTGRES_DB_PORT = env::var("POSTGRES_DB_PORT").expect("Error: Working directory environment variable POSTGRES_DB_PORT not found");
    let POSTGRES_DB_NAME = env::var("POSTGRES_DB_NAME").expect("Error: Working directory environment variable POSTGRES_DB_NAME not found");
    let POSTGRES_DB_USER = env::var("POSTGRES_DB_USER").expect("Error: Working directory environment variable POSTGRES_DB_USER not found");
    let POSTGRES_DB_PASSWORD = env::var("POSTGRES_DB_PASSWORD").expect("Error: Working directory environment variable POSTGRES_DB_PASSWORD not found");

    let connectionString = format!("postgres://{}:{}@{}:{}/{}",
        POSTGRES_DB_USER,
        POSTGRES_DB_PASSWORD,
        POSTGRES_DB_HOST,
        POSTGRES_DB_PORT,
        POSTGRES_DB_NAME);
    println!("{}", connectionString);

    // Print the value associated with the "PWD" key
    // let manager = PostgresConnectionManager::new(
    //     "host=localhost user=postgres".parse().unwrap(),
    //     NoTls,
    // );
    // let pool = r2d2::Pool::new(manager).unwrap();
    
    // for i in 0..1i32 {
    //     let pool = pool.clone();
    //     let query = r#"SELECT "listingId", "title", "description", "price", "inStock", "length", "width", "height", "imageUrl", "deleted", "createdOn" FROM "public"."listing" AS "ListingModel" WHERE "ListingModel"."deleted" = false;"#;
    //     thread::spawn(move || {
    //         let mut client = pool.get().unwrap();
    //         client.execute(query, &[&i]).unwrap();
    //     });
    // }
    
}