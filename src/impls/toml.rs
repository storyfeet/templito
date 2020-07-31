use crate::*;
use err::*;
use func_man::TFunc;
use std::str::FromStr;
use toml::Value;

impl Templable for Value {
    fn parse_lit(s: &str) -> anyhow::Result<Self> {
        let pstr = format!("x={}\n", s);
        match Value::from_str(&pstr) {
            Ok(Value::Table(m)) => m
                .get("x")
                .map(|v| v.clone())
                .ok_or(ea_str("No x in parse result")),
            Ok(_) => Err(ea_str("Toml parse result not a table")),
            Err(e) => Err(e.into()),
        }
    }
    fn string(s: &str) -> Self {
        Value::String(s.to_string())
    }
    fn bool(b: bool) -> Self {
        Value::Boolean(b)
    }
    fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None, //TODO consider other bits
        }
    }
    fn is_valid(&self) -> bool {
        match self {
            Value::Table(t) => t.len() == 0,
            Value::Array(a) => a.len() == 0,
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
        Value::Integer(u as i64)
    }
    fn as_usize(&self) -> Option<usize> {
        match self {
            Value::Integer(n) => Some(*n as usize),
            _ => None,
        }
    }

    fn keys(&self) -> Option<Vec<String>> {
        match self {
            Value::Table(m) => Some(m.keys().map(|v| v.to_string()).collect()),
            _ => None,
        }
    }

    fn get_key<'a>(&'a self, k: &str) -> Option<&'a Self> {
        match self {
            Value::Table(m) => m.get(k),
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
            _ => None,
        }
    }

    fn get_func<'a>(&'a self, s: &str) -> Option<&'a TFunc<Self>> {
        match s {
            "add" => Some(&|l| super::fold(l, add)),
            "sub" => Some(&|l| super::fold(l, sub)),
            "mul" => Some(&|l| super::fold(l, mul)),
            "div" => Some(&|l| super::fold(l, div)),
            "mod" => Some(&|l| super::fold(l, modulo)),
            _ => None,
        }
    }
}

fn add(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64) + b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + (*b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot add non numeric values")),
    }
}

fn sub(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64) - b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - (*b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}

fn mul(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64) * b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * (*b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot mul non numeric values")),
    }
}
fn div(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64) / b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a / (*b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot div non numeric values")),
    }
}
fn modulo(a: Value, b: &Value) -> anyhow::Result<Value> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64) % b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a % (*b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}

#[cfg(test)]
mod toml_test {
    use super::*;
    use crate::*;
    use pipeline::*;
    use std::str::FromStr;
    use template::*;
    #[test]
    fn test_toml_values_come_out_correct() {
        let tt = TreeTemplate::from_str(
            r#"{{let n='5';b='false';s="hat"}}{{if $b}}BTRUEERROR{{else}}{{$n}}>{{$0}}*{{$s}}{{/if}}"#,
        )
        .unwrap();
        let data = toml::Value::String("GOBBLE".to_string());
        let fm = func_man::default_func_man();
        let mut tm = temp_man::BasicTemps::new();
        let res = tt.run(&[data], &mut tm, &fm).unwrap();
        assert_eq!(res, "5>GOBBLE*hat");
    }
}
