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

pub fn ea_op<O>(o: Option<O>, s: &'static str) -> anyhow::Result<O> {
    o.ok_or(Error::Str(s).into())
}

#[derive(Debug, Clone, Error)]
#[error("{:?}, {}",.e,.s)]
pub struct WrapErr<E: std::error::Error + Debug + Clone> {
    pub e: Option<E>,
    pub s: String,
}

pub trait OpError {
    type Res;
    fn e_str(self, s: &'static str) -> Result<Self::Res, Error>;
}

impl<A> OpError for Option<A> {
    type Res = A;

    fn e_str(self, s: &'static str) -> Result<Self::Res, Error> {
        self.ok_or(Error::Str(s))
    }
}
