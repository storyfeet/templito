pub mod err;
pub mod func_man;
pub mod funcs;
//pub mod impls;
mod parser;
mod pipeline;
mod scope;
pub mod temp_man;
pub mod template;
mod tests;
use template::{TreeTemplate, VarPart};
pub mod prelude;
use std::collections::HashMap;

use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TData {
    Bool(bool),
    String(String),
    Int(i64),
    UInt(u64),
    Float(f64),
    List(Vec<TData>),
    Map(HashMap<String, TData>),
}

impl fmt::Display for TData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TData::*;
        match Self {
            Bool(b) => write!(f, "{}", b),
            String(s) => write!(f, "{}", s),
            Int(i) => write!(f, "{}", i),
            UInt(u) => write!(f, "{}", u),
            Float(f) => write!(f, "{}", f),
            List(v) => write!(f, "{:?}", v),
            Map(m) => write!(f, "{:?}", m),
        }
    }
}

pub trait TParam {
    fn get_s(s: &str) -> Option<TData>;
    fn get_u(u: usize) -> Option<TData>;
}

impl TData {
    ///How the type will be created from the template

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
            SV::Null => Self::Bool(false),
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
            TData::Bool(b) => Some(b),
            _ => None,
        }
    }

    ///Return the usize value that will be used for lookups and indexing
    fn as_usize(&self) -> Option<usize> {
        match self {
            TData::Usize(b) => Some(b),
            _ => None,
        }
    }

    ///Return a list of keys for for loops combined with
    fn keys(&self) -> Option<Vec<String>> {
        None
    }

    ///The len will be used in for loops when the value can be treated as an array
    ///Return None if the item cannot be indexed like an array
    fn len(&self) -> Option<usize> {
        None
    }
    ///This is used with 'keys' by for loops
    fn get_key<'a>(&'a self, _s: &str) -> Option<&'a Self> {
        None
    }
    ///This is used by for loops
    fn get_index<'a>(&'a self, _n: usize) -> Option<&'a Self> {
        None
    }

    ///This function exists to enable '$0.cat.3' indexing
    fn get_var_part<'a>(&'a self, v: &VarPart) -> Option<&'a Self> {
        match v {
            VarPart::Num(n) => self.get_index(*n),
            VarPart::Id(s) => self.get_key(s),
        }
    }

    fn get_var_path<'a>(&'a self, v: &[VarPart]) -> Option<&'a Self> {
        if v.len() == 0 {
            return Some(self);
        }
        self.get_var_part(&v[0])
            .and_then(|p| p.get_var_path(&v[1..]))
    }
}
