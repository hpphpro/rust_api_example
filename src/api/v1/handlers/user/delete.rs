use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use crate::{
    api::common::helpers::try_transaction, 
    common::{
        error::{
            AppError, AppErrorMessage
        }, 
    structs::{
        requests::user::DeleteUser, 
        responses::{
            status::Status, user::User
        }
    }}, 
    database::entity::user::Role, services::gateway::get_gateway
};


pub async fn delete_user_handler(
    connection: &DatabaseConnection,
    user: User,
    body: DeleteUser,
) -> Result<Status, AppError> {
    let user_id: Uuid;

    if user.role == Role::Admin {
        user_id = body.id.or(Some(user.id)).expect("User id did not provided");
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
    
    let status = get_gateway(connection).user().delete(user_id).await;
    
    match status {
        Ok(result) => {
            try_transaction(transaction.commit().await, "Failed to delete a user. Commit error".into())?;
            Ok(result)
        },
        Err(error) => {
            try_transaction(transaction.rollback().await, "Failed to delete a user. Rollback error".into())?;
            Err(error)
        }
    }

}