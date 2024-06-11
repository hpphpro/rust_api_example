use log::info;

use axum::{middleware, Router};


use crate::api::common::middlewares::{error::error_handler, process_time::process_time};


pub fn setup_middlewares(router: Router) -> Router {
    info!("Setup global middlewares... ");
    router
        .layer(middleware::from_fn(process_time))
        .layer(middleware::from_fn(error_handler))
}