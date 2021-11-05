
use std::{env::{self, current_dir}, net::IpAddr, path::{Path}, sync::Arc};

use mongodb::{Client, options::ClientOptions, Database};
use dotenv::dotenv;
use rocket::{Rocket, fs::{FileServer, NamedFile}, get};

mod graphql;
mod guards;

use crate::graphql::{generate_schema, gql_playground_handler, gql_graphiql_handler, gql_handler};


async fn setup_db() -> Database {

    let mut client_options = ClientOptions::parse(std::env::var("MONGO_URL").expect("Mongo Url Environment Variable not found")).await.unwrap();

    client_options.app_name = Some("Rocket.rs Server".to_string());

    let client = Client::with_options(client_options).unwrap();

    log::info!("Connected successfully.");

    client.database("accounts")
}

fn static_files() -> FileServer {
    let path = format!("{}/{}", current_dir().unwrap().to_str().unwrap(), "public/");
    FileServer::from(path.as_str())
        .rank(1)
}


#[get("/<_..>", rank=2)]
async fn default_files() -> NamedFile {
    NamedFile::open(Path::new(format!("{}/{}", current_dir().unwrap().to_str().unwrap(), "public/index.html").as_str())).await.unwrap()
}

#[rocket::main]
async fn main() {
    if Path::new(".env").exists() {
        dotenv().ok();
    }

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());

    let schema = generate_schema();
    // let db = setup_db().await;

    let mut config = rocket::Config::default();
    config.port = port.parse().unwrap_or(3000);
    config.address = host.parse().unwrap_or(IpAddr::V4([127, 0, 0, 1].into()));


    log::info!("{:#?}", config);



    Rocket::custom(&config)
        // Services
        // .manage(Arc::new(db))
        .manage(schema)
        
        // Routes
        .mount("/api", rocket::routes![gql_handler, gql_playground_handler, gql_graphiql_handler])
        .mount("/", static_files())
        .mount("/", rocket::routes![default_files])
        
        .launch()
        .await.unwrap();
}