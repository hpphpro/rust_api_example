use std::sync::Arc;

use sea_orm::ConnectionTrait;

use crate::database::gateway::DBGateway;
use crate::services::user::UserService;

#[derive(Clone)]
pub struct ServiceGateway<'a, Conn> 
where Conn: ConnectionTrait + Send + Sync
{
    pub database: DBGateway<'a, Conn>
}

impl<'a, Conn> ServiceGateway<'a, Conn> 
where Conn: ConnectionTrait + Send + Sync
{
    fn new(database: DBGateway<'a, Conn>) -> Self {
        Self { database }
    }

    pub fn user(&self) -> Arc<UserService<Conn>> {
        UserService::new(self.database.user())
    }
}



pub fn get_gateway<'a, C>(conn: &'a C) -> ServiceGateway<'a, C>
where C: ConnectionTrait + Send + Sync
{
    ServiceGateway::new(DBGateway::new(conn))
}