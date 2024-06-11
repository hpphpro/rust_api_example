use std::sync::Arc;

use axum::extract::State;
use axum::http::header;
use axum::{
    extract::Request, 
    middleware::Next, 
    response::Response, 
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::api::v1::dependencies::AppState;
use crate::common::error::{AppError, AppErrorMessage};
use crate::services::gateway::get_gateway;



pub async fn auth(
    cookie_jar: CookieJar, 
    State(state): State<Arc<AppState>>, 
    mut request: Request, 
    next: Next
) -> Result<Response, AppError> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        }).ok_or_else(
            || {
                AppError::UnAuthorizedError(
                    AppErrorMessage { 
                        message: "Token is not provided".into(), 
                        details: None 
                    }
                )
            }/*  */
        )?;
    
    let claims = state.jwt.verify_token(token)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| {
            AppError::UnAuthorizedError(
                AppErrorMessage { message: "Invalid token".into(), details: None}
            )
        })?;
    let user = get_gateway(&*state.connection)
        .user()
        .get(user_id)
        .await
        .map_err(|_| AppError::UnAuthorizedError(
            AppErrorMessage {
                message: "Unauthorized".into(),
                details: None
            }
        ))?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)

}