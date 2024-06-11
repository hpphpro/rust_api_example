use std::sync::Arc;

use log::info;

use sea_orm::DatabaseConnection;

use crate::database::connection::{connection_options, make_connection};
use crate::core::config::Config;
use crate::services::security::{
    hash::{get_argon2_default, Argon2Hasher},
    jwt::{get_jwt, JWT},
};
use migration::{Migrator, MigratorTrait};

pub struct AppState {
    pub connection: Arc<DatabaseConnection>,
    pub hasher: Arc<Argon2Hasher>,
    pub config: Config,
    pub jwt: Arc<JWT>,
}

pub async fn run_migrations(connection: &DatabaseConnection) -> () {
    info!("Start db migrations... ");
    Migrator::status(connection).await.expect("Check migration status failed");
    Migrator::up(connection, None).await.expect("Migrations failed");
}


pub async fn setup_dependencies(config: Config) -> Arc<AppState> {
    info!("Setup dependencies... ");
    let connection = make_connection(connection_options(config.db.clone())).await;
    let hasher = get_argon2_default();
    let jwt = Arc::new(get_jwt(config.token.clone()));
    run_migrations(&connection).await;

    Arc::new(AppState { connection, hasher, config, jwt })
}