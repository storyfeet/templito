pub mod err;
pub mod func_man;
pub mod funcs;
mod parser;
mod pipeline;
mod scope;
pub mod temp_man;
pub mod template;
mod tests;
pub mod tparam;
use template::{TreeTemplate, VarPart};
pub mod prelude;
use std::cmp::Ordering;
use std::collections::HashMap;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TData {
    Bool(bool),
    String(String),
    Int(i64),
    UInt(usize),
    Float(f64),
    List(Vec<TData>),
    Map(HashMap<String, TData>),
    Null,
}

impl PartialOrd for TData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use TData::*;
        match (self, other) {
            (String(a), String(b)) => a.partial_cmp(b),
            (Int(a), Int(b)) => a.partial_cmp(b),
            (UInt(a), UInt(b)) => a.partial_cmp(b),
            (Float(a), Float(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl fmt::Display for TData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TData::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            String(s) => write!(f, "{}", s),
            Int(i) => write!(f, "{}", i),
            UInt(u) => write!(f, "{}", u),
            Float(fv) => write!(f, "{}", fv),
            List(v) => write!(f, "{:?}", v),
            Map(m) => write!(f, "{:?}", m),
            Null => write!(f, "NULL"),
        }
    }
}

impl TData {
    ///How the type will be created from the template
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let v = s.parse::<serde_json::Value>()?;
        Ok(Self::from_json(v))
    }

    fn from_json(v: serde_json::Value) -> Self {
        use serde_json::Value as SV;
        match v {
            SV::String(s) => Self::String(s),
            SV::Number(n) => {
                if n.is_f64() {
                    Self::Float(n.as_f64().unwrap())
                } else {
                    Self::Int(n.as_i64().unwrap())
                }
            }
            SV::Null => Self::Null,
            SV::Bool(b) => Self::Bool(b),
            SV::Array(a) => Self::List(a.into_iter().map(|v| TData::from_json(v)).collect()),
            SV::Object(m) => Self::Map(
                m.into_iter()
                    .map(|(k, v)| (k, TData::from_json(v)))
                    .collect(),
            ),
        }
    }

    ///Will be used for binary logic
    fn as_bool(&self) -> Option<bool> {
        match self {
            TData::Bool(b) => Some(*b),
            _ => None,
        }
    }

    ///Return the usize value that will be used for lookups and indexing
    fn as_usize(&self) -> Option<usize> {
        match self {
            TData::UInt(b) => Some(*b),
            _ => None,
        }
    }

    ///The len will be used in for loops when the value can be treated as an array
    ///Return None if the item cannot be indexed like an array
    fn len(&self) -> Option<usize> {
        None
    }
    ///This is used with 'keys' by for loops
    fn get_key<'a>(&'a self, s: &str) -> Option<&'a Self> {
        match self {
            TData::Map(m) => m.get(s),
            _ => None,
        }
    }
    ///This is used by for loops
    fn get_index<'a>(&'a self, n: usize) -> Option<&'a Self> {
        match self {
            TData::List(l) => l.get(n),
            _ => None,
        }
    }
}
