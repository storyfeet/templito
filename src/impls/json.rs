use crate::*;
use err::*;
use func_man::TFunc;
use serde_json::{Number, Value};
use std::str::FromStr;

impl Templable for Value {
    fn parse_lit(s: &str) -> anyhow::Result<Self> {
        ea_res(Value::from_str(s))
    }
    fn string(s: &str) -> Self {
        Value::String(s.to_string())
    }
    fn bool(b: bool) -> Self {
        Value::Bool(b)
    }
    fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None, //TODO consider other bits
        }
    }
    fn is_valid(&self) -> bool {
        match self {
            Value::Null => false,
            _ => true,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }
    fn usize(u: usize) -> Self {
        Value::Number(serde_json::Number::from(u))
    }
    fn as_usize(&self) -> Option<usize> {
        match self {
            Value::Number(n) => n.as_u64().map(|n| n as usize),
            _ => None,
        }
    }

    fn keys(&self) -> Option<Vec<String>> {
        match self {
            Value::Object(m) => Some(m.keys().map(|v| v.to_string()).collect()),
            _ => None,
        }
    }

    fn get_key<'a>(&'a self, k: &str) -> Option<&'a Self> {
        match self {
            Value::Object(m) => m.get(k),
            _ => None,
        }
    }

    fn len(&self) -> Option<usize> {
        match self {
            Value::Array(a) => Some(a.len()),
            _ => None,
        }
    }

    fn get_index<'a>(&'a self, n: usize) -> Option<&'a Self> {
        match self {
            Value::Array(a) => a.get(n),
            _ => None,
        }
    }

    fn compare(&self, b: &Self) -> Option<std::cmp::Ordering> {
        match (self, b) {
            (Value::String(sa), Value::String(sb)) => sa.partial_cmp(sb),
            (Value::Number(na), Value::Number(nb)) => (na.as_f64()).partial_cmp(&nb.as_f64()),
            _ => return None,
        }
    }

    fn get_func<'a>(&'a self, k: &str) -> Option<&'a TFunc<Self>> {
        match k {
            "to_json" => Some(&to_json),
            "add" => Some(&|l| super::fold(l, add)),
            "sub" => Some(&|l| super::fold(l, sub)),
            "mul" => Some(&|l| super::fold(l, mul)),
            "div" => Some(&|l| super::fold(l, div)),
            "mod" => Some(&|l| super::fold(l, modulo)),
            _ => None,
        }
    }
}

fn nums_f(a: &Number, b: &Number) -> Option<(f64, f64)> {
    match (a.is_f64() || b.is_f64(), a.as_f64(), b.as_f64()) {
        (true, Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
fn nums_i(a: &Number, b: &Number) -> Option<(i64, i64)> {
    match (a.as_i64(), b.as_i64()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn nums_u(a: &Number, b: &Number) -> Option<(u64, u64)> {
    match (a.as_u64(), b.as_u64()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn add(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Number(ref an), Value::Number(bn)) => {
            if let Some((af, bf)) = nums_f(an, bn) {
                Ok(Value::from(af + bf))
            } else if let Some((ai, bi)) = nums_i(an, bn) {
                Ok(Value::from(ai + bi))
            } else if let Some((au, bu)) = nums_u(an, bn) {
                Ok(Value::from(au + bu))
            } else {
                Err(ea_str("Nums could not be reconciled for adding"))
            }
        }
        _ => Err(ea_str("Add only works on numbers Operator")),
    }
}
fn sub(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Number(ref an), Value::Number(bn)) => {
            if let Some((af, bf)) = nums_f(an, bn) {
                Ok(Value::from(af - bf))
            } else if let Some((ai, bi)) = nums_i(an, bn) {
                Ok(Value::from(ai - bi))
            } else if let Some((au, bu)) = nums_u(an, bn) {
                Ok(Value::from(au - bu))
            } else {
                Err(ea_str("Nums could not be reconciled for adding"))
            }
        }
        _ => Err(ea_str("Add only works on numbers Operator")),
    }
}
fn div(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Number(ref an), Value::Number(bn)) => {
            if let Some((af, bf)) = nums_f(an, bn) {
                Ok(Value::from(af / bf))
            } else if let Some((ai, bi)) = nums_i(an, bn) {
                Ok(Value::from(ai / bi))
            } else if let Some((au, bu)) = nums_u(an, bn) {
                Ok(Value::from(au / bu))
            } else {
                Err(ea_str("Nums could not be reconciled for adding"))
            }
        }
        _ => Err(ea_str("Add only works on numbers Operator")),
    }
}

fn mul(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Number(ref an), Value::Number(bn)) => {
            if let Some((af, bf)) = nums_f(an, bn) {
                Ok(Value::from(af * bf))
            } else if let Some((ai, bi)) = nums_i(an, bn) {
                Ok(Value::from(ai * bi))
            } else if let Some((au, bu)) = nums_u(an, bn) {
                Ok(Value::from(au * bu))
            } else {
                Err(ea_str("Nums could not be reconciled for adding"))
            }
        }
        _ => Err(ea_str("Add only works on numbers Operator")),
    }
}

fn modulo(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Number(ref an), Value::Number(bn)) => {
            if let Some((af, bf)) = nums_f(an, bn) {
                Ok(Value::from(af % bf))
            } else if let Some((ai, bi)) = nums_i(an, bn) {
                Ok(Value::from(ai % bi))
            } else if let Some((au, bu)) = nums_u(an, bn) {
                Ok(Value::from(au % bu))
            } else {
                Err(ea_str("Nums could not be reconciled for adding"))
            }
        }
        _ => Err(ea_str("Add only works on numbers Operator")),
    }
}

fn to_json(l: &[Value]) -> anyhow::Result<Value> {
    let rs = match l.len() {
        1 => serde_json::to_string(&l[0])?,
        _ => serde_json::to_string(l)?,
    };
    Ok(Value::String(rs))
}
