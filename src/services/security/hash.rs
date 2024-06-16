use std::sync::Arc;

use argon2::{
    password_hash::{
        rand_core::OsRng, SaltString
    }, 
    Algorithm, 
    Argon2, 
    Params, 
    PasswordHash, 
    PasswordHasher, 
    PasswordVerifier, 
    Version
};

use crate::common::error::{AppError, AppErrorMessage};


#[derive(Clone)]
pub struct Argon2Hasher {
    pub argon2: Argon2<'static>
}


impl Argon2Hasher {

    fn new(algorithm: Option<Algorithm>, version: Option<Version>, params: Option<Params>) -> Self {
        let argon2 = Argon2::new(
            algorithm.unwrap_or(Algorithm::default()), 
            version.unwrap_or(Version::default()),
            params.unwrap_or(Params::default())
        );

        Self {
            argon2
        }
    }

    pub fn hash_password(&self, plain_text: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed = self.argon2
            .hash_password(plain_text.as_bytes(), &salt)
            .map_err(|_| AppError::BadRequestError(AppErrorMessage { message: "Failed to hash password".into(), details: None }))?;

        Ok(hashed.to_string())
    }

    pub fn verify_password(&self, hashed_text: &str, plain_text: &str) -> bool {
        let password_hash = PasswordHash::new(&hashed_text);

        if let Ok(pwd) = password_hash {
            self.argon2.verify_password(&plain_text.as_bytes(), &pwd).is_ok()
        } else {
            false
        }
    }
}



pub fn get_argon2_default() -> Arc<Argon2Hasher> {
    Arc::new(Argon2Hasher::new(None, None, None))
}