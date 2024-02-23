use std::error::Error;

use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use super::model::User;

pub trait UserRepository<C>
    where C : AsyncConnection
{
    async fn load_by_username(connection: &mut C, username: &str) -> Result<Option<User>, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct UserRepositoryPostgres;

impl UserRepository<AsyncPgConnection> for UserRepositoryPostgres {

    async fn load_by_username(connection: &mut AsyncPgConnection, username: &str) -> Result<Option<User>, Box<dyn Error>> {
        Ok(None)
    }

}