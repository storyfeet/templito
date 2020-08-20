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
    Template(TreeTemplate),
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
            Template(_t) => write!(f, "TEMPLATE"),
        }
    }
}

impl TData {
    ///How the type will be created from the template
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let v = s.parse::<serde_json::Value>()?;
        Ok(Self::from_json(v))
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            TData::String(s) => Some(s),
            _ => None,
        }
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

    pub fn from_toml(v: toml::Value) -> Self {
        use toml::Value as TV;
        match v {
            TV::String(s) => Self::String(s),
            TV::Integer(n) => Self::Int(n),
            TV::Float(f) => Self::Float(f),
            TV::Boolean(b) => Self::Bool(b),
            TV::Array(a) => Self::List(a.into_iter().map(|v| TData::from_toml(v)).collect()),
            TV::Table(m) => Self::Map(
                m.into_iter()
                    .map(|(k, v)| (k, TData::from_toml(v)))
                    .collect(),
            ),
            TV::Datetime(dt) => Self::String(dt.to_string()),
        }
    }

    ///Will be used for binary logic
    fn as_bool(&self) -> Option<bool> {
        match self {
            TData::Bool(b) => Some(*b),
            TData::String(s) => Some(s.len() > 0),
            TData::UInt(u) => Some(*u > 0),
            TData::Int(i) => Some(*i != 0),
            TData::Float(f) => Some(*f != 0.),
            _ => None,
        }
    }

    ///Return the usize value that will be used for lookups and indexing
    fn as_usize(&self) -> Option<usize> {
        match self {
            TData::UInt(b) => Some(*b),
            TData::Int(n) if *n >= 0 => Some(*n as usize),
            _ => None,
        }
    }

    pub fn get_key_str<'a>(&'a self, k: &str) -> Option<&'a str> {
        match self {
            TData::Map(m) => match m.get(k) {
                Some(TData::String(ref s)) => Some(s),
                _ => None,
            },
            _ => None,
        }
    }
    pub fn get_key_string(&self, k: &str) -> Option<String> {
        match self {
            TData::Map(m) => m.get(k).map(|d| d.to_string()),
            _ => None,
        }
    }
}
