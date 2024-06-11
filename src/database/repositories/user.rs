#![allow(unused)]

use sea_orm::sea_query::OnConflict;
use sea_orm::{
    prelude::*, 
    ActiveValue, 
    QuerySelect, 
    TryInsertResult,
};

#[allow(unused)]
use crate::database::repositories::base::IntoActiveModel;
use crate::into_active_model;
use crate::database::entity::user::{self, ActiveModel, Entity as User, Model, Role};
use super::base::Repository;

use core::result::Result::Ok;


#[derive(Debug)]
pub struct NewUser {
    pub login: String,
    pub password: String
}

pub struct UpdateUser {
    pub id: Uuid,
    pub login: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>
}

pub struct DeleteUser {
    pub id: Uuid
}

into_active_model!(UpdateUser, ActiveModel, { mandatory: id }, { optional: login, optional: password, optional: role });
into_active_model!(DeleteUser, ActiveModel, { mandatory: id }, {});

impl IntoActiveModel for NewUser {
    type Model = ActiveModel;

    fn into_active_model(self) -> ActiveModel {
        let mut model = ActiveModel::new();

        model.login = ActiveValue::Set(self.login);
        model.password = ActiveValue::Set(self.password);

        model
    }
}

pub struct Writer<'a, Conn: ConnectionTrait> {
    conn: &'a Conn
}

impl<'a, Conn: ConnectionTrait> Writer<'a, Conn> {
    pub async fn create(&self, new_user: NewUser) -> Result<Model, anyhow::Error> {     
        let user = new_user.into_active_model().insert(self.conn).await?;
    
        Ok(user)
    }

    pub async fn create_many(&self, users: Vec<NewUser>) -> Result<TryInsertResult<Model>, anyhow::Error> {
        let models: Vec<ActiveModel> = users.into_iter()
            .map(|new| new.into_active_model())
            .collect();
        
        let result = User::insert_many(models)
            .on_empty_do_nothing()
            .on_conflict(OnConflict::column(user::Column::Login).do_nothing().to_owned())
            .exec_with_returning(self.conn)
            .await?;
        
        Ok(result)
    }  

    pub async fn update(&self, update_user: UpdateUser) -> Result<Model, anyhow::Error> {
        let user = update_user.into_active_model().update(self.conn).await?;
        Ok(user)
    }

    pub async fn delete(&self, delete_user: DeleteUser) -> Result<u64, anyhow::Error> {
        let user = delete_user.into_active_model().delete(self.conn).await?;

        Ok(user.rows_affected)
    }
}

pub struct Reader<'a, Conn: ConnectionTrait> {
    conn: &'a Conn
}

impl<'a, Conn: ConnectionTrait> Reader<'a, Conn> {
    pub async fn get(&self, id: Uuid) -> Result<Option<Model>, anyhow::Error> {
        let user = User::find_by_id(id).one(self.conn).await?;
        Ok(user)
        
    }

    pub async fn get_by_login(&self, login: String) -> Result<Option<Model>, anyhow::Error> {
        let user = User::find().filter(user::Column::Login.eq(login)).one(self.conn).await?;

        Ok(user)
    }

    pub async fn get_many(&self, offset: Option<u64>, limit: Option<u64>) -> Result<Vec<Model>, anyhow::Error>  {
        let user = User::find()
            .offset(offset)
            .limit(limit)
            .all(self.conn)
            .await?;
        Ok(user)
    }
    pub async fn count(&self) -> Result<u64, anyhow::Error> {
        let count = User::find().count(self.conn).await?;
        Ok(count)
    }

    pub async fn exists(&self, id: Uuid) -> Result<bool, anyhow::Error> {
        let user = self.get(id).await?;
        Ok(user.is_some())
    }
}

#[derive(Clone)]
pub struct UserRepository<'a, Conn: ConnectionTrait> {
    conn:  &'a Conn
}

#[async_trait::async_trait]
impl<'a, Conn> Repository<'a, Conn> for UserRepository<'a, Conn>
where
    Conn: ConnectionTrait + Send + Sync
{

    fn new(conn: &'a Conn) -> Self {
        Self { conn }
    }
    fn connection(&self) -> &'a Conn {
        self.conn
    }
}


impl<'a, Conn: ConnectionTrait + Send + Sync> UserRepository<'a, Conn> {

    pub fn writer(&self) -> Writer<'a, Conn> {
        Writer { conn: self.connection() }
    }

    pub fn reader(&self) -> Reader<'a, Conn> {
        Reader { conn: self.connection() }
    }
}