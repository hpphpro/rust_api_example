use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::database::entity::user::Role;


#[derive(Clone, Serialize, ToSchema)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000", format = "Uuid")]
    pub id: Uuid,
    pub login: String,
    pub role: Role,
    #[schema(example = "2023-05-15T13:45:30Z", format = "date-time")]
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, ToSchema)]
pub struct UserData {
    pub total: u64,
    pub data: Vec<User>
}