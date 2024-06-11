#![allow(unused)]

use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use thiserror::Error;
use serde_json::{Value, Serializer};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct AppErrorMessage {
    pub message: Box<str>,
    pub details: Option<Value>
}


#[derive(Error, Debug)]
pub enum AppError {
    UnAuthorizedError(AppErrorMessage),
    ConflictError(AppErrorMessage),
    ForbiddenError(AppErrorMessage),
    NotFoundError(AppErrorMessage),
    BadRequestError(AppErrorMessage),
    TooManyRequestsError(AppErrorMessage),
    ServiceUnavailableError(AppErrorMessage),
    ServiceNotImplementedError(AppErrorMessage),
    UnprocessableEntityError(AppErrorMessage),
    InternalServerError(AppErrorMessage),
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequestError(err) => write!(f, "{}", err.message),
            AppError::ConflictError(err) => write!(f, "{}", err.message),
            AppError::ForbiddenError(err) => write!(f, "{}", err.message),
            AppError::NotFoundError(err) => write!(f, "{}", err.message),
            AppError::UnAuthorizedError(err) => write!(f, "{}", err.message),
            AppError::TooManyRequestsError(err) => write!(f, "{}", err.message),
            AppError::ServiceUnavailableError(err) => write!(f, "{}", err.message),
            AppError::ServiceNotImplementedError(err) => write!(f, "{}", err.message),
            AppError::UnprocessableEntityError(err) => write!(f, "{}", err.message),
            AppError::InternalServerError(err) => write!(f, "{}", err.message),
            AppError::UnknownError(err) => write!(f, "{}", err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequestError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ConflictError(msg) => (StatusCode::CONFLICT, msg),
            AppError::ForbiddenError(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFoundError(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::UnAuthorizedError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::TooManyRequestsError(msg) => (StatusCode::TOO_MANY_REQUESTS, msg),
            AppError::UnprocessableEntityError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::ServiceUnavailableError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            AppError::ServiceNotImplementedError(msg) => (StatusCode::NOT_IMPLEMENTED, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::UnknownError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, AppErrorMessage { message: "Unknown".into(), details: None })
            }
        };
        (status, Json(message)).into_response()
    }
}
