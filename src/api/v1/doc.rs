use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, SecurityScheme, HttpBuilder};


use crate::common::error::AppErrorMessage;

use crate::api::v1::endpoints::healthcheck::__path_healthcheck_endpoint;
use crate::api::v1::endpoints::user::{
    __path_create_user_endpoint, 
    __path_get_user_by_id_endpoint, 
    __path_get_many_users_endpoint, 
    __path_update_user_endpoint,
    __path_delete_user_endpoint,
    __path_get_me_endpoint,
};
use crate::api::v1::endpoints::auth::{
    __path_login_endpoint,
    __path_logout_endpoint,
    __path_refresh_endpoint,
};
use crate::common::structs::requests::user::{CreateUser, DeleteUser, LoginUser, UpdateUser};
use crate::common::structs::responses::healthcheck::HealthCheck;
use crate::common::structs::responses::status::Status;
use crate::common::structs::responses::token::{Token, TokenType};
use crate::common::structs::responses::user::{User, UserData};
use crate::database::entity::user::Role;


struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let security_scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build()
        );
        if let Some(components) = &mut openapi.components {
            components.security_schemes.insert(
                "jwt_token".to_string(),
                security_scheme
            );
        } else {
            openapi.components = Some(
                utoipa::openapi::ComponentsBuilder::new()
                    .security_scheme(
                        "jwt_token",
                        security_scheme
                    )
                    .build(),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        healthcheck_endpoint, 
        login_endpoint,
        logout_endpoint,
        refresh_endpoint,
        create_user_endpoint,
        get_me_endpoint,
        get_many_users_endpoint,
        get_user_by_id_endpoint,
        update_user_endpoint,
        delete_user_endpoint,
    ), 
    components(
        schemas(
            HealthCheck, 
            CreateUser, 
            UpdateUser,
            User, 
            AppErrorMessage, 
            Role, 
            UserData,
            LoginUser,
            Token,
            Status,
            TokenType,
            DeleteUser
        ),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;