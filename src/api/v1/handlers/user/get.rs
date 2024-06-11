use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::common::structs::responses::user::UserData;
use crate::common::{error::AppError, structs::responses::user::User};
use crate::services::gateway::get_gateway;


pub async fn get_user(connection: Arc<DatabaseConnection>, user_id: Uuid) -> Result<User, AppError> {
    let gw = get_gateway(&*connection);

    let service = gw.user();

    service.get(user_id).await

}


pub async fn get_many_users(
    connection: &DatabaseConnection, offset: Option<u64>, limit: Option<u64>
) -> Result<UserData, AppError> {
    let gw = get_gateway(connection);

    gw.user().get_many(offset, limit).await
}