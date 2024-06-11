use axum::{
    extract::Request, 
    http::StatusCode, 
    middleware::Next, 
    response::Response, 
};

pub async fn process_time(request: Request, next: Next) -> Result<Response, StatusCode> {
    
    let start = tokio::time::Instant::now();

    let mut response = next.run(request).await;

    let end = start.elapsed().as_secs_f64();

    response.headers_mut().insert("X-Process-Time", end.to_string().parse().unwrap()); 

    Ok(response)
}