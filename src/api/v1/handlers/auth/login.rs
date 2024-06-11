
use axum::{body::Body, http::{header, HeaderValue, Response, StatusCode}, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use sea_orm::DatabaseConnection;
use time::OffsetDateTime;

use crate::{
    common::{error::{AppError, AppErrorMessage}, 
    structs::{requests::user::LoginUser, responses::token::TokenType}}, 
    services::{gateway::get_gateway, security::jwt::JWT}
};
use crate::services::security::hash::Argon2Hasher;



pub async fn login_handler(
    connection: &DatabaseConnection,
    hasher: &Argon2Hasher,
    jwt: &JWT,
    login_user: LoginUser,
) -> Result<Response<Body>, AppError> {
    
    let user = get_gateway(connection)
        .user()
        .reader
        .get_by_login(login_user.login.into_string())
        .await?;


    if let Some(user) = user {
        let verify = hasher.verify_password(&user.password, &login_user.password);
        if !verify {
            return Err(AppError::BadRequestError(
                AppErrorMessage { 
                    message: "Invalid password".into(), 
                    details: None 
                }
            ));
        }
        let user_id = user.id.to_string();
        let (_, access) = jwt.create_token(user_id.clone(), TokenType::ACCESS, None)?;
        let (exp, refresh) = jwt.create_token(user_id, TokenType::REFRESH, None)?;


        let cookie = Cookie::build(("refresh", refresh.token))
            .expires(
                OffsetDateTime::from_unix_timestamp(exp as i64)
                .map_err(|_| {
                    AppError::InternalServerError(
                        AppErrorMessage {
                            message: "Failed to set expires token date".into(),
                            details: None
                        }
                    )
                })?
            )
            .path("/")
            .max_age(time::Duration::seconds(exp as i64))
            .same_site(SameSite::Strict)
            .http_only(true)
            .build();

        let mut response = (StatusCode::OK, Json(access)).into_response();
        response.headers_mut().insert(
            header::SET_COOKIE, 
            HeaderValue::from_str(&cookie.to_string()).map_err(|_| {
                AppError::InternalServerError(
                    AppErrorMessage {
                        message: "Could not set cookie".into(),
                        details: None
                    }
                )
            })?
        );

        Ok(response)
        
    } else {
        return Err(AppError::NotFoundError(
            AppErrorMessage {
                message: "User not found".into(),
                details: None
            }
        ));
    }

}