use aws_sdk_secretsmanager::{Client, Error as AWSError};

#[derive(Debug)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}


async fn get_secret(client: &Client, name: &str) -> Result<String, AWSError> {
    let resp = client.get_secret_value().secret_id(name).send().await?;
    Ok(resp.secret_string().unwrap_or("No value!").to_string())
}


use serde_json::Error;

pub enum ConnectionType {
    Environment,
    SecretsManager,
}

pub async fn getConnection() -> Result<ConnectionInfo, serde_json::Error> {
    use crate::connect::get_secret;
    use aws_sdk_secretsmanager::Client;
    use aws_config::meta::region::RegionProviderChain;
    use std::env;
    use dotenv::dotenv;
    use serde_json::{Value};

    dotenv().ok();
    let connection_type =ConnectionType::SecretsManager;
    match connection_type {
        ConnectionType::Environment => {
            let postgres_db_host = env::var("POSTGRES_DB_HOST").expect("Error: Working directory environment variable POSTGRES_DB_HOST not found");
            let postgres_db_port = env::var("POSTGRES_DB_PORT").expect("Error: Working directory environment variable POSTGRES_DB_PORT not found");
            let postgres_db_name = env::var("POSTGRES_DB_NAME").expect("Error: Working directory environment variable POSTGRES_DB_NAME not found");
            let postgres_db_user = env::var("POSTGRES_DB_USER").expect("Error: Working directory environment variable POSTGRES_DB_USER not found");
            let postgres_db_password = env::var("POSTGRES_DB_PASSWORD").expect("Error: Working directory environment variable POSTGRES_DB_PASSWORD not found");
            Ok(ConnectionInfo {
                host: postgres_db_host,
                port: postgres_db_port.parse::<u16>().unwrap(),
                username: postgres_db_user,
                password: postgres_db_password,
                database: postgres_db_name,
            })
        },
        ConnectionType::SecretsManager => {
            let secret_name = env::var("SECRET_NAME").expect("Error: Working directory environment variable POSTGRES_DB_SECRET_NAME not found");
            let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

            let shared_config = aws_config::from_env().region(region_provider).load().await;
            let client = Client::new(&shared_config);
            let secret = get_secret(&client, &secret_name).await;
            let parsedSecret = match secret {
                Ok(secret) => {
                    let parsed: Value  = serde_json::from_str(&secret)?;
                    parsed
                },
                Err(error) => panic!("Problem parsing the secret: {:?}", error),
            };
            Ok(ConnectionInfo {
                host: parsedSecret["host"].to_string().replace("\"", "").replace("\\", ""),
                port: parsedSecret["port"].to_string().replace("\"", "").replace("\\", "").parse::<u16>().unwrap(),
                username: parsedSecret["username"].to_string().replace("\"", "").replace("\\", ""),
                password: parsedSecret["password"].to_string().replace("\"", "").replace("\\", ""),
                database: parsedSecret["dbname"].to_string().replace("\"", "").replace("\\", ""),
            })
        },
    }
}