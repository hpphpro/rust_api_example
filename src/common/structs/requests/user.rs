use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;


use crate::database::entity::user::Role;


#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub login: Box<str>,
    pub password: Box<str>,
    pub confirm_password: Box<str>
}

#[derive(Deserialize, ToSchema)]
pub struct LoginUser {
    pub login: Box<str>,
    pub password: Box<str>
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteUser {
    pub id: Option<Uuid>
}


#[derive(Deserialize, ToSchema)]
pub struct UpdateUser {
    pub id: Option<Uuid>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>
}

impl CreateUser {
    
    pub fn check_password(&self) -> bool {
        self.password == self.confirm_password
    }
}


