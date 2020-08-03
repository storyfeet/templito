use crate::err::*;
use crate::*;

pub fn fold<T: Clone, F: Fn(T, &T) -> anyhow::Result<T>>(l: &[T], f: F) -> anyhow::Result<T> {
    if l.len() == 0 {
        return Err(ea_str("No arguments given").into());
    }
    let mut res = l[0].clone();
    for a in &l[1..] {
        res = f(res, a)?;
    }
    Ok(res)
}

pub enum Match {
    U(u64, u64),
    I(i64, i64),
    F(f64, f64),
}

fn num_match(a: &TData, b: &TData) -> Option<Match> {
    use Match::*;
    match (a, b) {
        (TData::UInt(a), TData::UInt(b)) => Some(U(*a, *b)),
        (TData::UInt(a), TData::Int(b)) => I(*a as i64, b),
        (TData::UInt(a), TData::Float(b)) => F(a as f64, b),
        (TData::Int(a), TData::UInt(b)) => I(a, b),
        (TData::Int(a), TData::Int(b)) => U(a, b),
        (TData::Int(a), TData::Float(b)) => F(a, b),
        (TData::Float(a), TData::UInt(b)) => F(a, b),
        (TData::Float(a), TData::Int(b)) => F(a, b),
        (TData::Float(a), TData::Float(b)) => F(a, b),
    }
}

fn add(l: &[TData]) -> anyhow::Result<TData> {
    fold(l, |a, b| match (a, b) {
        (TData::UInt(a), TData::UInt(b)) => Ok(TData::UInt(a + b)),
        (TData::Int(a), TData::Int(b)) => Ok(TData::Int(a + b)),
        (TData::Int(a), TData::Float(b)) => Ok(TData::Float((a as f64) + b)),
        (TData::Float(a), TData::Int(b)) => Ok(TData::Float(a + (*b as f64))),
        (TData::Float(a), TData::Float(b)) => Ok(TData::Float(a + b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}

fn sub(a: TData, b: &TData) -> anyhow::Result<TData> {
    match (a, b) {
        (TData::Int(a), TData::Int(b)) => Ok(TData::Int(a - b)),
        (TData::Int(a), TData::Float(b)) => Ok(TData::Float((a as f64) - b)),
        (TData::Float(a), TData::Int(b)) => Ok(TData::Float(a - (*b as f64))),
        (TData::Float(a), TData::Float(b)) => Ok(TData::Float(a - b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}

fn mul(a: TData, b: &TData) -> anyhow::Result<TData> {
    match (a, b) {
        (TData::Int(a), TData::Int(b)) => Ok(TData::Int(a * b)),
        (TData::Int(a), TData::Float(b)) => Ok(TData::Float((a as f64) * b)),
        (TData::Float(a), TData::Int(b)) => Ok(TData::Float(a * (*b as f64))),
        (TData::Float(a), TData::Float(b)) => Ok(TData::Float(a * b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot mul non numeric values")),
    }
}
fn div(a: TData, b: &TData) -> anyhow::Result<TData> {
    match (a, b) {
        (TData::Int(a), TData::Int(b)) => Ok(TData::Int(a / b)),
        (TData::Int(a), TData::Float(b)) => Ok(TData::Float((a as f64) / b)),
        (TData::Float(a), TData::Int(b)) => Ok(TData::Float(a / (*b as f64))),
        (TData::Float(a), TData::Float(b)) => Ok(TData::Float(a / b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot div non numeric values")),
    }
}
fn modulo(a: TData, b: &TData) -> anyhow::Result<TData> {
    match (a, b) {
        (TData::Int(a), TData::Int(b)) => Ok(TData::Int(a % b)),
        (TData::Int(a), TData::Float(b)) => Ok(TData::Float((a as f64) % b)),
        (TData::Float(a), TData::Int(b)) => Ok(TData::Float(a % (*b as f64))),
        (TData::Float(a), TData::Float(b)) => Ok(TData::Float(a % b)),
        //TODO consider allowing Date tweaks
        _ => Err(ea_str("Cannot sub non numeric values")),
    }
}
