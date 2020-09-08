extern crate diesel;
extern crate mobc;

#[macro_use]
extern crate async_trait;

use diesel::{Connection, ConnectionError};
use mobc::Manager as ManageConnection;
use std::convert::Into;
use std::fmt;
use std::marker::PhantomData;

pub struct ConnectionManager<T> {
    database_url: String,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send + 'static> Sync for ConnectionManager<T> {
}

impl<T> ConnectionManager<T> {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        ConnectionManager {
            database_url: database_url.into(),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConnectionError(ref e) => e.fmt(f),
            Error::QueryError(ref e) => e.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConnectionError(ref e) => e.description(),
            Error::QueryError(ref e) => e.description(),
        }
    }
}

#[async_trait]
impl<T> ManageConnection for ConnectionManager<T> where
    T: Connection + Send + 'static,
{
    type Connection = T;
    type Error = Error;

    async fn connect(&self) -> Result<T, Error> {
        T::establish(&self.database_url)
            .map_err(Error::ConnectionError)
    }

    async fn check(&self, conn: T) -> Result<T, Error> {
        conn.execute("SELECT 1").map(|_| conn).map_err(Error::QueryError)
    }
}
