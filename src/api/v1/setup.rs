use log::info;

use axum::{middleware, routing::{delete, get, post}, Router};

use crate::{
    api::v1::{
        dependencies::setup_dependencies, 
        endpoints::{
            auth::{
                login_endpoint, logout_endpoint
            }, 
        healthcheck::healthcheck_endpoint, 
        user::{
            delete_user_endpoint, get_me_endpoint, update_user_endpoint
        }}, 
        middlewares::auth::auth
    }, 
    core::config::Config
};
use crate::api::v1::endpoints::user::{create_user_endpoint, get_user_by_id_endpoint, get_many_users_endpoint};

pub async fn create_v1_router(config: Config) -> Router {
    info!("Creating v1 router... ");
    let state = setup_dependencies(config).await;
    let auth_middleware = middleware::from_fn_with_state(state.clone(), auth);
    let router = Router::new()
       .route(
        "/healthcheck",
        get(healthcheck_endpoint)
        )
       .route("/users", 
        post(create_user_endpoint)
        )
       .route("/users", 
        get(get_many_users_endpoint).patch(update_user_endpoint).route_layer(auth_middleware.clone())
        )
       .route("/users/:user_id", 
        get(get_user_by_id_endpoint).route_layer(auth_middleware.clone())
        )
       .route("/users", delete(delete_user_endpoint).route_layer(auth_middleware.clone()))
       .route("/users/me", get(get_me_endpoint).route_layer(auth_middleware.clone()))
       .route("/auth/login", post(login_endpoint))
       .route("/auth/logout", post(logout_endpoint).route_layer(auth_middleware.clone()))
       .with_state(state);

    

    Router::new().nest("/v1", router)
    
}