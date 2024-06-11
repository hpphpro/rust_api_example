use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};


use crate::{
    api::v1::{
        dependencies::AppState, 
        handlers::auth::{login::login_handler, logout::logout_handler}
    }, common::structs::requests::user::LoginUser, 
};


#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginUser,
    responses(
        (
            status = 200,
            description = "Success",
            body = Token
        ),
        (
            status = 400,
            description = "Bad Request",
            body = AppErrorMessage,
            example = json!({"message": "Password mismatch", "details": null})
        ),
        (
            status = 404,
            description = "Not Found",
            body = AppErrorMessage,
            example = json!({"message": "User Not found", "details": null})
        ),
        (
            status = 500,
            description = "Internal Server Error",
            body = AppErrorMessage,
            example = json!({"message": "Unknown", "details": null})
        )
    )
)]
pub async fn login_endpoint(
    State(state): State<Arc<AppState>>, 
    Json(body): Json<LoginUser>
) -> impl IntoResponse {
    match login_handler(&state.connection, &state.hasher, &state.jwt, body).await {
        Ok(response) => response,
        Err(error) => error.into_response()
    }
}



#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses(
        (
            status = 200,
            description = "Success",
            body = Status
        ),
        (
            status = 500,
            description = "Internal Server Error",
            body = AppErrorMessage,
            example = json!({"message": "Unknown", "details": null})
        )
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn logout_endpoint() -> impl IntoResponse {
    match logout_handler().await {
        Ok(response) => response,
        Err(error) => error.into_response()
    }
}