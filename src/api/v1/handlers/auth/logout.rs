use axum::{body::Body, http::{header, HeaderValue, Response, StatusCode}, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use time::OffsetDateTime;

use crate::common::{error::{AppError, AppErrorMessage}, structs::responses::status::Status};



pub async fn logout_handler() -> Result<Response<Body>, AppError> {
    let cookie = Cookie::build(("refresh", ""))
        .expires(
            OffsetDateTime::from_unix_timestamp(-1)
            .map_err(|_| {
                AppError::InternalServerError(
                    AppErrorMessage {
                        message: "Failed to unset expires token date".into(),
                        details: None
                    }
                )
            })?
        )
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Strict)
        .http_only(true)
        .build();

    let mut response = (StatusCode::OK, Json(Status { status: true })).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).map_err(|_| {
            AppError::InternalServerError(
                AppErrorMessage {
                    message: "Failed to remove cookie".into(),
                    details: None
                }
            )
        })?
    );

    Ok(response)

}