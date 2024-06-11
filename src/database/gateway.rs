use std::sync::Arc;

use sea_orm::ConnectionTrait;

use crate::database::repositories::user::UserRepository;

use crate::database::repositories::base::Repository;

#[derive(Clone)]
pub struct DBGateway<'a, Conn: ConnectionTrait + Send + Sync> {
    conn: &'a Conn
}

impl<'a, Conn> DBGateway<'a, Conn>
where Conn: ConnectionTrait + Send + Sync
{
    pub fn new(conn: &'a Conn) -> Self {
        Self { conn }
    }

    pub fn user(&self) -> Arc<UserRepository<Conn>> {
        Arc::new(UserRepository::new(self.conn))
    }
}