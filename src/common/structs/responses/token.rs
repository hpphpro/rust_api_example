use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum TokenType {
    ACCESS,
    REFRESH
}


#[derive(Serialize, ToSchema)]
pub struct Token {
    pub typ: TokenType,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub _type: TokenType,
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}