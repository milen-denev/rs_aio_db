use std::fmt;

pub mod aio_database;
pub(crate) mod internal;
pub mod models;
pub mod aio_query;

#[derive(Debug)]
pub(crate) enum _WalMode {
     WAL,
     WAL2
}

impl fmt::Display for _WalMode {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{:?}", self)
     }
 }