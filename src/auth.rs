use core::fmt;
use std::error::Error;

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use diesel_async::pooled_connection::deadpool::{self, PoolError};

use crate::core::AsyncPgConnectionPool;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User {
    id: i32,
    token: Vec<u8>
}

impl AuthUser for User {

    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.token
    }

}

#[derive(Clone)]
pub struct PostgresBackend {
    pool: AsyncPgConnectionPool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginCredentials {
    username: String,
    password: String
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoginError {
    CannotConnectToDatabase(PoolError),
    InvalidUsernameOrPassword,
    MissingPermission
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LoginError::CannotConnectToDatabase(pool_error) => write!(f, "Cannot connect to the database"),
            LoginError::InvalidUsernameOrPassword => write!(f, "The username or the password are invalid"),
            LoginError::MissingPermission => write!(f, "You don't have the required permissions")
        }
    }
}

impl Error for LoginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let LoginError::CannotConnectToDatabase(pool_error) = self {
            Some(pool_error)
        } else {
            None
        }
    }
}

#[async_trait]
impl AuthnBackend for PostgresBackend {

    type User = User;
    type Credentials = LoginCredentials;
    type Error = LoginError;

    async fn authenticate(
        &self,
        LoginCredentials { username, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Get db connection
        let mut connection = self.pool.get().await
            .map_err(|error| LoginError::CannotConnectToDatabase(error))?;
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        
    }

}