use axum::body::{to_bytes, Body};
use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::Request, 
    http::StatusCode, 
    middleware::Next, 
    response::Response, 
};

use serde_json::json;

async fn handle_error<T, E>(result: Result<T, E>) -> Result<T, StatusCode> {
    result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


pub async fn error_handler(request: Request, next: Next) -> Result<Response, StatusCode> {
    
    let response = next.run(request).await;

    if response.status() == StatusCode::BAD_REQUEST 
        || response.status() == StatusCode::UNPROCESSABLE_ENTITY 
    {
        let (parts, body) = response.into_parts();
        let body = handle_error(to_bytes(body, usize::MAX).await).await?;

        let is_json_like = serde_json::from_slice::<serde_json::Value>(&body).is_ok();

        let builder = Response::builder()
            .status(parts.status)
            .header(CONTENT_TYPE, "application/json");
        if is_json_like {
            let rebuild_response = handle_error(builder.body(Body::from(body))).await?;
            return Ok(rebuild_response);
        }
        let body_string = handle_error(String::from_utf8(body.to_vec())).await?;
            

        let message = handle_error(serde_json::to_string(&json!({ "message": body_string, "details": null }))).await?;
        let rebuild_response = handle_error(builder.body(Body::from(message))).await?;

        return Ok(rebuild_response);
    }

    Ok(response)
}