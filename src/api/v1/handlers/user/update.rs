use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use crate::api::common::helpers::try_transaction;
use crate::common::error::AppErrorMessage;
use crate::common::structs::requests::user::UpdateUser;
use crate::common::{error::AppError, structs::responses::user::User};
use crate::database::entity::user::Role;
use crate::services::gateway::get_gateway;
use crate::services::security::hash::Argon2Hasher;


pub async fn update_user(
    connection: &DatabaseConnection, user: User, data: UpdateUser, hasher: &Argon2Hasher
) -> Result<User, AppError> {
    let user_id: Uuid;

    if user.role == Role::Admin {
        user_id = data.id.or(Some(user.id)).expect("User id did not provided");
    } else {
        user_id = user.id;
    }

    let transaction = connection
        .begin()
        .await
        .map_err(|_| {
            AppError::BadRequestError(
                AppErrorMessage { 
                    message: "Failed to open transaction".into(), 
                    details: None 
                })
        })?;
    let gw = get_gateway(&transaction);

    let user = gw.user().update(user_id, data, hasher).await;

    match user {
        Ok(result) => {
            try_transaction(transaction.commit().await, "Failed to update a user. Commit error".into())?;
            Ok(result)
        },
        Err(error) => {
            try_transaction(transaction.rollback().await, "Failed to update a user. Rollback error".into())?;
            Err(error)
        }
    }


}
