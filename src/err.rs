use std::fmt::Debug;
use thiserror::*;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{}",.0)]
    Str(&'static str),
    #[error("{}",.0)]
    String(String),
}

#[derive(Debug, Clone, Error)]
#[error("{:?}, {}",.e,.s)]
pub struct WrapErr<E: std::error::Error + Debug + Clone> {
    pub e: Option<E>,
    pub s: String,
}
