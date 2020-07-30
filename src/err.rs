use std::fmt::Debug;
use thiserror::*;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{}",.0)]
    Str(&'static str),
    #[error("{}",.0)]
    String(String),
}

pub fn ea_str(s: &'static str) -> anyhow::Error {
    Error::Str(s).into()
}

pub fn ea_string(s: String) -> anyhow::Error {
    Error::String(s).into()
}

pub fn ea_res<O, E: 'static + std::error::Error + Sync + Send>(
    r: Result<O, E>,
) -> anyhow::Result<O> {
    r.map_err(|e| e.into())
}

#[derive(Debug, Clone, Error)]
#[error("{:?}, {}",.e,.s)]
pub struct WrapErr<E: std::error::Error + Debug + Clone> {
    pub e: Option<E>,
    pub s: String,
}
