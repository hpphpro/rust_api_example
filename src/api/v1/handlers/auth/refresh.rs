use axum::{
    body::Body, 
    http::{
        header, HeaderValue, Response, StatusCode
    }, 
    response::IntoResponse, Json
};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use time::OffsetDateTime;

use crate::{
    common::{
        error::{AppError, AppErrorMessage}, 
        structs::responses::token::TokenType
}, 
    services::security::jwt::JWT
};




pub async fn refresh_handler(
    jwt: &JWT,
    cookie_jar: CookieJar,
) -> Result<Response<Body>, AppError> {
    let token = cookie_jar
        .get("refresh")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(
            || {
                AppError::UnAuthorizedError(
                    AppErrorMessage {
                        message: "Token is not provided".into(),
                        details: None
                    }
                )
            }
        )?;
    
    let claims = jwt.verify_token(token)?;

    if claims._type != TokenType::REFRESH {
        return Err(AppError::UnAuthorizedError(
            AppErrorMessage {
                message: "Invalid token".into(),
                details: None
            }
        ));
    }

    let (_, access) = jwt.create_token(claims.sub.clone(), TokenType::ACCESS, None)?;
    let (exp, refresh) = jwt.create_token(claims.sub, TokenType::REFRESH, None)?;

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
        cookie.to_string().parse::<HeaderValue>().map_err( |_|
            AppError::InternalServerError(
                AppErrorMessage {
                    message: "Could not set cookie".into(),
                    details: None
                }
            )
        )?
    );

    Ok(response)

}