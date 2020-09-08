use crate::*;
use err::*;
use tdata::*;
use tparam::*;

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

pub enum NumMatch {
    U(usize, usize),
    I(isize, isize),
    F(f64, f64),
}
use NumMatch::*;

pub fn num_match(a: &TData, b: &TData) -> Option<NumMatch> {
    Some(match (a, b) {
        (TData::UInt(a), TData::UInt(b)) => U(*a, *b),
        (TData::UInt(a), TData::Int(b)) => I(*a as isize, *b),
        (TData::UInt(a), TData::Float(b)) => F(*a as f64, *b),
        (TData::Int(a), TData::UInt(b)) => I(*a, *b as isize),
        (TData::Int(a), TData::Int(b)) => I(*a, *b),
        (TData::Int(a), TData::Float(b)) => F(*a as f64, *b),
        (TData::Float(a), TData::UInt(b)) => F(*a, *b as f64),
        (TData::Float(a), TData::Int(b)) => F(*a, *b as f64),
        (TData::Float(a), TData::Float(b)) => F(*a, *b),
        _ => return None,
    })
}

pub fn add<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TBoco::Co(TData::UInt(a + b))),
        Some(F(a, b)) => Ok(TBoco::Co(TData::Float(a + b))),
        Some(I(a, b)) => Ok(TBoco::Co(TData::Int(a + b))),
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}

pub fn sub<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => {
            if a >= b {
                Ok(TBoco::Co(TData::UInt(a - b)))
            } else {
                Ok(TBoco::Co(TData::Int(a as isize - b as isize)))
            }
        }
        Some(F(a, b)) => Ok(TBoco::Co(TData::Float(a - b))),
        Some(I(a, b)) => Ok(TBoco::Co(TData::Int(a - b))),
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}

pub fn mul<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TBoco::Co(TData::UInt(a * b))),
        Some(F(a, b)) => Ok(TBoco::Co(TData::Float(a * b))),
        Some(I(a, b)) => Ok(TBoco::Co(TData::Int(a * b))),
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}

pub fn div<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TBoco::Co(TData::UInt(a / b))),
        Some(F(a, b)) => Ok(TBoco::Co(TData::Float(a / b))),
        Some(I(a, b)) => Ok(TBoco::Co(TData::Int(a / b))),
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}

pub fn modulo<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TBoco::Co(TData::UInt(a % b))),
        Some(F(a, b)) => Ok(TBoco::Co(TData::Float(a % b))),
        Some(I(a, b)) => Ok(TBoco::Co(TData::Int(a % b))),
        _ => Err(ea_str("Cannot add non numeric values")),
    })
}
