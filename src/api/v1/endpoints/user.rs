use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use uuid::Uuid;

use crate::api::v1::dependencies::AppState;
use crate::api::v1::handlers::user::delete::delete_user_handler;
use crate::api::v1::handlers::user::update::update_user;
use crate::common::structs::requests::pagination::Pagination;
use crate::common::structs::requests::user::{CreateUser, DeleteUser, UpdateUser};

use crate::api::v1::handlers::user::create::create_user;
use crate::api::v1::handlers::user::get::{get_user, get_many_users};
use crate::common::structs::responses::user::User;

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "user",
    request_body = CreateUser,
    responses(
        (
            status = 201,
            description = "User created successfully",
            body = User
        ),
        (
            status = 400,
            description = "Bad Request",
            body = AppErrorMessage,
            example = json!({"message": "Password mismatch", "details": null})
        ),
        (
            status = 409,
            description = "Conflict",
            body = AppErrorMessage,
            example = json!({"message": "User already exists", "details": null})
        ),
        (
            status = 500,
            description = "Internal Server Error",
            body = AppErrorMessage,
            example = json!({"message": "Unknown", "details": null})
        )
    )
)]
pub async fn create_user_endpoint(
    State(state): State<Arc<AppState>>, 
    Json(data): Json<CreateUser>,
) -> impl IntoResponse {
    match create_user(&state.connection, data, &state.hasher).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(error) => error.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "user",
    params(
        Pagination
    ),
    responses(
        (
            status = 200,
            description = "Successfully",
            body = UserData
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
pub async fn get_many_users_endpoint(
    State(state): State<Arc<AppState>>, 
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let (offset, limit) = pagination.calculate_offset_and_limit();

    match get_many_users(&state.connection, Some(offset), Some(limit)).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(error) => error.into_response()
    }
}


#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    tag = "user",
    params(
        ("user_id" = Uuid, description = "Unique identifier of the user")
    ),
    responses(
        (
            status = 200,
            description = "Success",
            body = User
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
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_user_by_id_endpoint(
    State(state): State<Arc<AppState>>, 
    Path(user_id): Path<Uuid>
) -> impl IntoResponse {
    match get_user(state.connection.clone(), user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(error) => error.into_response()
    }
}



#[utoipa::path(
    patch,
    path = "/api/v1/users",
    tag = "user",
    request_body = UpdateUser,
    responses(
        (
            status = 200,
            description = "User updated successfully",
            body = User
        ),
        (
            status = 400,
            description = "Bad Request",
            body = AppErrorMessage,
            example = json!({"message": "Transaction failed", "details": null})
        ),
        (
            status = 409,
            description = "Conflict",
            body = AppErrorMessage,
            example = json!({"message": "Login already exists", "details": null})
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
pub async fn update_user_endpoint(
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>,
    Json(data): Json<UpdateUser>,
) -> impl IntoResponse {
    match update_user(&state.connection, user, data, &state.hasher).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(error) => error.into_response()
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/users",
    tag = "user",
    request_body = DeleteUser,
    responses(
        (
            status = 200,
            description = "User deleted successfully",
            body = Status
        ),
        (
            status = 400,
            description = "Bad Request",
            body = AppErrorMessage,
            example = json!({"message": "Transaction failed", "details": null})
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
pub async fn delete_user_endpoint(
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>,
    Json(data): Json<DeleteUser>,
) -> impl IntoResponse {
    match delete_user_handler(&state.connection, user, data).await {
        Ok(status) => (StatusCode::OK, Json(status)).into_response(),
        Err(error) => error.into_response()
    }
}


#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "user",
    responses(
        (
            status = 200,
            description = "Success",
            body = User
        ),
        (
            status = 404,
            description = "Not Found",
            body = AppErrorMessage,
            example = json!({"message": "User Not found", "details": null})
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = AppErrorMessage,
            example = json!({"message": "Unauthorized", "details": null})
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
pub async fn get_me_endpoint(
    Extension(user): Extension<User>
) -> impl IntoResponse {
    (StatusCode::OK, Json(user)).into_response()
}