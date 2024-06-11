use std::error::Error;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use log::info;
use simple_logger::SimpleLogger;

use dotenv::dotenv;

use tower_http::cors::CorsLayer;
mod api;
pub mod core;
mod common;
mod database;
mod services;
use crate::api::setup::create_general_router;
use crate::api::v1::setup::create_v1_router;
use crate::core::config::Config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().ok();
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    info!("Getting config... ");
    let config = Config::new();

    info!("Creating router... ");
    let cors = CorsLayer::new()
        .allow_origin(format!("http://{}:{}", config.server.host(), config.server.port()).parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_general_router(
        vec![create_v1_router(config.clone()).await], 
    )
        .await
        .layer(cors);

    info!("Starting server... ");
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", config.server.host(), config.server.port())
    ).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}
