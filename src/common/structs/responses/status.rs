use serde::Serialize;
use utoipa::ToSchema;



#[derive(Serialize, ToSchema)]
pub struct Status {
    pub status: bool
}