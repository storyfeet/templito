use crate::*;
use err::*;
use func_man::TFunc;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

pub trait Wrapable: 'static + Sized + PartialEq + Debug + Clone {
    fn get_func(_: &str) -> Option<&'static TFunc<TWrap<Self>>> {
        None
    }
    //fn compare(&self, b: &Self) -> Option<std::cmp::Ordering>;
    fn get_data(&self, s: &str) -> Option<TWrap<Self>>;
    fn get_t_data(&self, t: &TWrap<Self>) -> Option<TWrap<Self>> {
        match t {
            TWrap::String(s) => self.get_data(s),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TWrap<T> {
    IType(T),
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
    List(Vec<TWrap<T>>),
    Map(HashMap<String, TWrap<T>>),
}

impl<T: Wrapable> Display for TWrap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IType(t) => write!(f, "{:?}", t),
            Self::Bool(t) => write!(f, "{}", t),
            Self::Int(t) => write!(f, "{}", t),
            Self::String(t) => write!(f, "{}", t),
            Self::Float(t) => write!(f, "{}", t),
            Self::List(t) => write!(f, "{:?}", t),
            Self::Map(t) => write!(f, "{:?}", t),
        }
    }
}

impl<T: Wrapable> TWrap<T> {
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
            SV::Array(a) => Self::List(a.into_iter().map(|v| TWrap::from_json(v)).collect()),
            SV::Object(m) => Self::Map(
                m.into_iter()
                    .map(|(k, v)| (k, TWrap::from_json(v)))
                    .collect(),
            ),
        }
    }
}

impl<T: Wrapable> Templable for TWrap<T> {
    fn parse_lit(s: &str) -> anyhow::Result<Self> {
        Ok(TWrap::from_json(s.parse::<serde_json::Value>()?))
    }

    fn string(s: &str) -> Self {
        TWrap::String(s.to_string())
    }

    fn bool(b: bool) -> Self {
        TWrap::Bool(b)
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            TWrap::Bool(b) => Some(*b),
            _ => None, //TODO consider other bits
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            TWrap::String(ref s) => Some(s),
            _ => None,
        }
    }

    fn usize(u: usize) -> Self {
        TWrap::Int(u as i64)
    }
    fn as_usize(&self) -> Option<usize> {
        match self {
            TWrap::Int(i) => Some(*i as usize),
            _ => None,
        }
    }

    fn keys(&self) -> Option<Vec<String>> {
        match self {
            TWrap::Map(m) => Some(m.keys().map(|v| v.to_string()).collect()),
            _ => None,
        }
    }

    fn get_key<'a>(&'a self, k: &str) -> Option<&'a Self> {
        match self {
            TWrap::Map(m) => m.get(k),
            _ => None,
        }
    }

    fn len(&self) -> Option<usize> {
        match self {
            TWrap::List(a) => Some(a.len()),
            _ => None,
        }
    }

    fn get_index<'a>(&'a self, n: usize) -> Option<&'a Self> {
        match self {
            TWrap::List(a) => a.get(n),
            _ => None,
        }
    }

    fn compare(&self, b: &Self) -> Option<std::cmp::Ordering> {
        match (self, b) {
            (TWrap::String(sa), TWrap::String(sb)) => sa.partial_cmp(sb),
            (TWrap::Int(na), TWrap::Int(nb)) => na.partial_cmp(nb),
            (TWrap::Float(na), TWrap::Float(nb)) => na.partial_cmp(nb),
            _ => return None,
        }
    }

    fn get_func(k: &str) -> Option<&'static TFunc<Self>> {
        match k {
            "add" => Some(&|l| super::fold(l, add)),
            "sub" => Some(&|l| super::fold(l, sub)),
            "mul" => Some(&|l| super::fold(l, mul)),
            "div" => Some(&|l| super::fold(l, div)),
            "mod" => Some(&|l| super::fold(l, modulo)),
            _ => None,
        }
    }
    fn list(v: Vec<Self>) -> Option<Self> {
        Some(Self::List(v))
    }
}

fn add<T: Wrapable>(a: TWrap<T>, b: &TWrap<T>) -> anyhow::Result<TWrap<T>> {
    match (a, b) {
        (TWrap::Int(a), TWrap::Int(b)) => Ok(TWrap::Int(a + b)),
        (TWrap::Int(a), TWrap::Float(b)) => Ok(TWrap::Float((a as f64) + b)),
        (TWrap::Float(a), TWrap::Int(b)) => Ok(TWrap::Float(a + (*b as f64))),
        (TWrap::Float(a), TWrap::Float(b)) => Ok(TWrap::Float(a + b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot add non numeric values")),
    }
}

fn sub<T: Wrapable>(a: TWrap<T>, b: &TWrap<T>) -> anyhow::Result<TWrap<T>> {
    match (a, b) {
        (TWrap::Int(a), TWrap::Int(b)) => Ok(TWrap::Int(a - b)),
        (TWrap::Int(a), TWrap::Float(b)) => Ok(TWrap::Float((a as f64) - b)),
        (TWrap::Float(a), TWrap::Int(b)) => Ok(TWrap::Float(a - (*b as f64))),
        (TWrap::Float(a), TWrap::Float(b)) => Ok(TWrap::Float(a - b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}

fn mul<T: Wrapable>(a: TWrap<T>, b: &TWrap<T>) -> anyhow::Result<TWrap<T>> {
    match (a, b) {
        (TWrap::Int(a), TWrap::Int(b)) => Ok(TWrap::Int(a * b)),
        (TWrap::Int(a), TWrap::Float(b)) => Ok(TWrap::Float((a as f64) * b)),
        (TWrap::Float(a), TWrap::Int(b)) => Ok(TWrap::Float(a * (*b as f64))),
        (TWrap::Float(a), TWrap::Float(b)) => Ok(TWrap::Float(a * b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot mul non numeric values")),
    }
}
fn div<T: Wrapable>(a: TWrap<T>, b: &TWrap<T>) -> anyhow::Result<TWrap<T>> {
    match (a, b) {
        (TWrap::Int(a), TWrap::Int(b)) => Ok(TWrap::Int(a / b)),
        (TWrap::Int(a), TWrap::Float(b)) => Ok(TWrap::Float((a as f64) / b)),
        (TWrap::Float(a), TWrap::Int(b)) => Ok(TWrap::Float(a / (*b as f64))),
        (TWrap::Float(a), TWrap::Float(b)) => Ok(TWrap::Float(a / b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot div non numeric values")),
    }
}
fn modulo<T: Wrapable>(a: TWrap<T>, b: &TWrap<T>) -> anyhow::Result<TWrap<T>> {
    match (a, b) {
        (TWrap::Int(a), TWrap::Int(b)) => Ok(TWrap::Int(a % b)),
        (TWrap::Int(a), TWrap::Float(b)) => Ok(TWrap::Float((a as f64) % b)),
        (TWrap::Float(a), TWrap::Int(b)) => Ok(TWrap::Float(a % (*b as f64))),
        (TWrap::Float(a), TWrap::Float(b)) => Ok(TWrap::Float(a % b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}
