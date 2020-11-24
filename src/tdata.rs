use crate::funcs::math;
use gobble::traits::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use crate::template::TreeTemplate;
#[derive(Clone, Debug)]
pub enum TData {
    Bool(bool),
    String(String),
    Int(isize),
    UInt(usize),
    Float(f64),
    List(Vec<TData>),
    Map(HashMap<String, TData>),
    Template(TreeTemplate),
    Null,
    Date(chrono::NaiveDate),
    Bytes(Vec<u8>),
}

impl PartialOrd for TData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use math::NumMatch::*;
        use TData::*;
        match math::num_match(self, other) {
            Some(I(a, b)) => return a.partial_cmp(&b),
            Some(U(a, b)) => return a.partial_cmp(&b),
            Some(F(a, b)) => return a.partial_cmp(&b),
            None => {}
        }
        match (self, other) {
            (String(a), String(b)) => return a.partial_cmp(b),
            (Date(a), Date(b)) => return a.partial_cmp(b),
            _ => {}
        }
        self.mode_rank().partial_cmp(&other.mode_rank())
    }
}

impl PartialEq for TData {
    fn eq(&self, other: &Self) -> bool {
        use math::NumMatch::*;
        use TData::*;
        match math::num_match(self, other) {
            Some(I(a, b)) => return a == b,
            Some(U(a, b)) => return a == b,
            Some(F(a, b)) => return a == b,
            None => {}
        }
        match (self, other) {
            (String(a), String(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Date(a), Date(b)) => a == b,
            (Null, Null) => true,
            _ => false,
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
            List(v) => {
                let mut coma = "";
                write!(f, "[")?;
                for i in v {
                    write!(f, "{}{}", coma, i)?;
                    coma = ",";
                }
                write!(f, "]")
            }
            Map(m) => write!(f, "{:?}", m),
            Null => write!(f, "NULL"),
            Template(_t) => write!(f, "TEMPLATE"),
            Date(d) => write!(f, "{}", d.format("%d/%m/%Y")),
            Bytes(b) => write!(f, "b\"{:?}\"", b),
        }
    }
}

impl From<String> for TData {
    fn from(s: String) -> TData {
        TData::String(s)
    }
}

impl From<usize> for TData {
    fn from(u: usize) -> TData {
        TData::UInt(u)
    }
}
impl From<&str> for TData {
    fn from(s: &str) -> TData {
        TData::String(s.to_string())
    }
}
impl From<&[&str]> for TData {
    fn from(v: &[&str]) -> TData {
        TData::List(v.into_iter().map(|t| TData::from(*t)).collect())
    }
}
impl From<&[String]> for TData {
    fn from(v: &[String]) -> TData {
        TData::List(v.into_iter().map(|t| TData::String(t.clone())).collect())
    }
}

impl TData {
    ///How the type will be created from the template
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        crate::parse::expr::BasicData
            .parse_s(s)
            .map_err(|e| e.strung().into())
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            TData::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn mode_rank(&self) -> usize {
        use TData::*;
        match self {
            Null => 0,
            Template(_) => 1,
            Map(_) => 2,
            List(_) => 3,
            Bool(_) => 4,
            Int(_) => 5,
            UInt(_) => 6,
            Date(_) => 7,
            Float(_) => 8,
            String(_) => 9,
            Bytes(_) => 10,
        }
    }

    pub fn from_json(v: serde_json::Value) -> Self {
        use serde_json::Value as SV;
        match v {
            SV::String(s) => Self::String(s),
            SV::Number(n) => {
                if n.is_f64() {
                    Self::Float(n.as_f64().unwrap())
                } else {
                    Self::Int(n.as_i64().unwrap() as isize)
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
            TV::Integer(n) => Self::Int(n as isize),
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
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TData::Bool(b) => Some(*b),
            TData::String(s) => Some(s.len() > 0),
            TData::UInt(u) => Some(*u > 0),
            TData::Int(i) => Some(*i != 0),
            TData::Float(f) => Some(*f != 0.),
            TData::List(l) => Some(l.len() > 0),
            TData::Map(m) => Some(m.len() > 0),
            TData::Null => Some(false),
            _ => None,
        }
    }

    ///Return the usize value that will be used for lookups and indexing
    pub fn as_usize(&self) -> Option<usize> {
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
