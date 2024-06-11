use std::{sync::Arc, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::core::config::DBConfig;


pub fn connection_options(config: DBConfig) -> ConnectOptions {
    let mut options = ConnectOptions::new(config.url());
    options
        .max_connections(config.max_connections())
        .min_connections(config.min_connections())
        .connect_timeout(Duration::from_secs(config.connect_timeout()))
        .acquire_timeout(Duration::from_secs(config.acquire_timeout()))
        .idle_timeout(Duration::from_secs(config.idle_timeout()))
        .max_lifetime(Duration::from_secs(config.max_lifetime()))
        .sqlx_logging(config.sqlx_logging())
        .sqlx_logging_level(config.sqlx_logging_level());
    

    options
}


pub async fn make_connection(options: ConnectOptions) -> Arc<DatabaseConnection> {
    let connection = Database::connect(options).await.unwrap();

    Arc::new(connection)
}


