use serde_json::json;

use crate::common::error::{AppError, AppErrorMessage};



pub fn try_transaction<T, E>(result: Result<T, E>, on_failure_message: Box<str>) -> Result<T, AppError> {
    result.map_err(
        |_| AppError::BadRequestError(
                AppErrorMessage { 
                    message: on_failure_message, 
                    details: json!({"message": "Transaction failed"}).into() 
                }
        )
    )
}