use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::{
    api::common::helpers::try_transaction, 
    common::{error::{AppError, AppErrorMessage}, structs::requests::user::CreateUser}, 
    services::{gateway::get_gateway, security::hash::Argon2Hasher}
};
use crate::common::structs::responses::user::User;



pub async fn create_user(connection: &DatabaseConnection, data: CreateUser, hasher: &Argon2Hasher) -> Result<User, AppError> {
    let transaction = connection
        .begin()
        .await
        .map_err(|_| AppError::BadRequestError(AppErrorMessage { message: "Failed to open transaction".into(), details: None }))?;
    let gateway = get_gateway(&transaction);

    let user = gateway.user().create(data, hasher).await;

    match user {
        Ok(result) => {
            try_transaction(transaction.commit().await, "Failed to create a user. Commit error".into())?;
            Ok(result)
        },
        Err(error) => {
            try_transaction(transaction.rollback().await, "Failed to create a user. Rollback error".into())?;
            Err(error)
        }
    }
}