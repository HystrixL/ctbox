
use std::result;

mod error;
pub mod network;

pub type Result<T,E = crate::error::Error> = result::Result<T,E>;