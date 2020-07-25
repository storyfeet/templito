use std::fmt::Debug;
use thiserror::*;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{}",.0)]
    Str(&'static str),
    #[error("{}",.0)]
    String(String),
}
