use std::pin::Pin;
use std::future::Future;

use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{Utc, DateTime};


#[derive(Clone, Debug, EnumIter, PartialEq, DeriveActiveEnum, Deserialize, Serialize, ToSchema)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(Some(128))")]
    pub login: String,
    #[sea_orm(column_type = "String(Some(255))")]
    pub password: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            role: ActiveValue::Set(Role::User),
            ..ActiveModelTrait::default()

        }
    }

    fn before_save<'life0, 'async_trait, C>(
        mut self,
        _: &'life0 C,
        insert: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Self, DbErr>> + Send + 'async_trait>>
    where
        C: ConnectionTrait + 'async_trait,
        'life0: 'async_trait,
        Self: Send + 'async_trait,
    {
        Box::pin(async move {
            if !insert {
                self.updated_at = ActiveValue::Set(Utc::now());
            }
        
            Ok(self)
        })
    }

}