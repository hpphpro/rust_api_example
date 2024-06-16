#![allow(unused)]

use std::sync::Arc;

use argon2::PasswordHash;
use sea_orm::ConnectionTrait;
use serde_json::json;
use uuid::Uuid;

use crate::common::structs::responses::status::Status;
use crate::database::repositories::user::{DeleteUser, NewUser, Reader, UpdateUser, UserRepository, Writer};
use crate::common::error::{AppError, AppErrorMessage};
use crate::common::structs::requests::user::{CreateUser, UpdateUser as UpdateUserRequest};
use crate::common::structs::responses::user::{User, UserData};

use super::security::hash::Argon2Hasher;

pub struct UserService<'a, Conn>
where Conn: ConnectionTrait + Send + Sync
{
    pub reader: Reader<'a, Conn>,
    pub writer: Writer<'a, Conn>
}

impl<'a, Conn> UserService<'a, Conn> 
where Conn: ConnectionTrait + Send + Sync
{
    pub fn new(repository: Arc<UserRepository<'a, Conn>>) -> Arc<Self> {
        let reader = repository.reader();
        let writer = repository.writer();
        Arc::new(Self { reader, writer })
    }

    pub async fn create(&self, mut data: CreateUser, hasher: &Argon2Hasher) -> Result<User, AppError> {

        data.password = hasher.hash_password(&data.password)?.into_boxed_str();
        
        let exists = self.reader.get_by_login(data.login.to_string()).await?;

        if exists.is_some() {
            return Err(AppError::ConflictError(AppErrorMessage { 
                message: "User already exists".into(), 
                details: json!({ "login": data.login }).into()
            }));
        }

        let model = self.writer
            .create(NewUser { login: data.login.to_string(), password: data.password.to_string() }).await?;
       
        Ok(User { 
            id: model.id, 
            login: model.login, 
            role: model.role, 
            created_at: model.created_at 
        })
        
    }

    pub async fn get(&self, id: Uuid) -> Result<User, AppError> {
        let user = self.reader.get(id).await?;

        if let Some(r) = user {
            return Ok(User { id: r.id, login: r.login, role: r.role, created_at: r.created_at});
        } else {
            Err(AppError::NotFoundError(AppErrorMessage { message: "User not found".into(), details: None}))
        }
    }

    pub async fn get_many(&self, offset: Option<u64>, limit: Option<u64>) -> Result<UserData, AppError> {

        let count = self.reader.count().await;

        match count {
            Ok(total) => {
                let models = self.reader.get_many(offset, limit).await?;
        
                let users = models
                    .into_iter()
                    .map(|model| User { id: model.id, login: model.login, role: model.role, created_at: model.created_at})
                    .collect();

                Ok(UserData {
                    total: total,
                    data: users
                })
            },
            Err(e) => {
                Ok(
                    UserData { total: 0, data: vec![] }
                )
            }
        }
    }

    pub async fn update(&self, id: Uuid, mut data: UpdateUserRequest, hasher: &Argon2Hasher) -> Result<User, AppError> {

        if let Some(login) = data.login.clone() {
            let exists = self.reader.get_by_login(login.to_string()).await?;

            if exists.is_some() {
                return Err(AppError::ConflictError(AppErrorMessage { 
                    message: "Login already exists".into(), 
                    details: json!({ "login": login }).into()
                }));
            }
        }
        if let Some(pwd) = data.password.clone() {
            data.password = Some(hasher.hash_password(&pwd)?);
        }

        let model = self.writer.update(
            UpdateUser { 
                id, 
                login: data.login, 
                password: data.password, 
                role: data.role
            }
            )
            .await?;

        Ok(User { 
            id: model.id, 
            login: model.login, 
            role: model.role, 
            created_at: model.created_at 
        })
  
    }

    pub async fn delete(&self, id: Uuid) -> Result<Status, AppError> {
        let exists = self.reader.exists(id).await?;

        if !exists {
            return Err(AppError::NotFoundError(
                AppErrorMessage {
                    message: "User not found".into(),
                    details: None
                }
            ));
        }

        let rows = self.writer.delete(DeleteUser { id }).await?;

        Ok(Status { status: rows > 0 })
    }

}