use serde::Serialize;
use utoipa::ToSchema;


#[derive(Serialize, ToSchema)]
pub struct HealthCheck {
    pub ok: bool
}