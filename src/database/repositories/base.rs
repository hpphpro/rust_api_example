use sea_orm::prelude::*;

#[async_trait::async_trait]
pub trait Repository<'a, Conn>
where Conn: Send + Sync + ConnectionTrait
{
    fn new(conn: &'a Conn) -> Self where Self: Sized;
    fn connection(&self) -> &'a Conn;
}

pub trait IntoActiveModel {
    type Model: Default;

    fn into_active_model(self) -> Self::Model;
}


