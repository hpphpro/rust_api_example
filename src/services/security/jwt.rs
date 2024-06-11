
use std::str::FromStr;

use chrono::TimeDelta;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use base64::prelude::*;

use crate::{common::{error::{AppError, AppErrorMessage}, structs::responses::token::{Token, TokenClaims, TokenType}}, core::config::TokenConfig};

#[derive(Clone)]
pub struct JWT {
    algorithm: Algorithm,
    secret_key: EncodingKey,
    public_key: DecodingKey,
    access_token_expire_seconds: i64,
    refresh_token_expire_seconds: i64,
}

impl JWT {
    fn new(
        algorithm: Box<str>, 
        secret_key: Box<str>, 
        public_key: Box<str>,  
        access_token_expire_seconds: i64, 
        refresh_token_expire_seconds: i64
    ) -> Result<Self, anyhow::Error> {
        
        let alg = Algorithm::from_str(&algorithm)?;
        let decoded_secret_key = BASE64_STANDARD.decode(secret_key.as_ref())?;
        let decoded_public_key = BASE64_STANDARD.decode(public_key.as_ref())?;
        let (public_key, secret_key) = match alg {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                (DecodingKey::from_secret(&decoded_public_key), EncodingKey::from_secret(&decoded_secret_key))
            },
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 | 
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 |
            Algorithm::ES256 | Algorithm::ES384 => {
                (DecodingKey::from_rsa_pem(&decoded_public_key)?, EncodingKey::from_rsa_pem(&decoded_secret_key)?)
            },
            Algorithm::EdDSA => {
                (DecodingKey::from_ed_pem(&decoded_public_key)?, EncodingKey::from_ed_pem(&decoded_secret_key)?)
            },
        };

        Ok(Self {
            algorithm: alg,
            secret_key,
            public_key,
            access_token_expire_seconds,
            refresh_token_expire_seconds,
        })
    }

    pub fn create_token(
        &self, sub: String, typ: TokenType, expire: Option<TimeDelta>
    ) -> Result<(usize, Token), AppError> {

        let now = chrono::Utc::now();

        let iat = now.timestamp() as usize;
        let exp = match typ {
            TokenType::ACCESS => {
                (now + expire.unwrap_or(chrono::Duration::seconds(self.access_token_expire_seconds)))
                    .timestamp() as usize
            }
            TokenType::REFRESH => {
                (now + expire.unwrap_or(chrono::Duration::seconds(self.refresh_token_expire_seconds)))
                    .timestamp() as usize
            }
        };

        if iat >= exp {
            return Err(AppError::ServiceNotImplementedError(
                AppErrorMessage { message: "Invalid expiration delta was provided".into(), details: None }
            ));
        }

        let token = encode(
            &Header::new(self.algorithm), 
            &TokenClaims {
                _type: typ.clone(),
                sub,
                iat,
                exp
            }, 
            &self.secret_key
        )
        .map_err(|_| {
            AppError::ServiceNotImplementedError(
                AppErrorMessage { 
                    message: "Failed to create a token".into(), 
                    details: None
            })
        })?;

        Ok((exp, Token { typ, token }))

    }

    pub fn verify_token(&self, token: String) -> Result<TokenClaims, AppError> {

        let data = decode::<TokenClaims>(
            &token, 
            &self.public_key,
            &Validation::new(self.algorithm)
        )
        .map_err(|_| {
            AppError::UnAuthorizedError(
                AppErrorMessage {
                    message: "Invalid token provided".into(),
                    details: None
                }
        )})?;

        Ok(data.claims)
    }
}



pub fn get_jwt(config: TokenConfig) -> JWT {
    JWT::new(
        config.algorithm, 
        config.secret_key, 
        config.public_key.expect("public_key must be set"), 
        config.access_token_expire_seconds, 
        config.refresh_token_expire_seconds
    ).expect("JWT service was not created")
}