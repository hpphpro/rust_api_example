use axum::Json;

use crate::common::structs::responses::healthcheck::HealthCheck;

#[utoipa::path(
    get, 
    path = "/api/v1/healthcheck",
    tag = "healthcheck", 
    responses(
        (
            status = 200,
            description = "Healthcheck",
            body = HealthCheck
        )
    )
)]
pub async fn healthcheck_endpoint() -> Json<HealthCheck> {
    Json(HealthCheck { ok: true })
}