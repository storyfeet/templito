use crate::*;
use err_tools::*;
use std::ops::Deref;
use tdata::*;
use tparam::*;

pub fn fold<T: Clone, F: Fn(T, &T) -> anyhow::Result<T>>(l: &[T], f: F) -> anyhow::Result<T> {
    if l.len() == 0 {
        return e_str("No arguments given");
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

pub fn add<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a + b))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a + b))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a + b))),
        _ => e_str("Cannot add non numeric values"),
    })
}

pub fn sub<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 1 {
        match l[0].deref() {
            TData::UInt(n) => return b_ok(TData::Int(-(*n as isize))),
            TData::Float(f) => return b_ok(TData::Float(-*f)),
            TData::Int(n) => return b_ok(TData::Int(-*n)),
            _ => return e_str("sub onl works on numbers"),
        }
    }
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => {
            if a >= b {
                Ok(TCow::Owned(TData::UInt(a - b)))
            } else {
                Ok(TCow::Owned(TData::Int(a as isize - b as isize)))
            }
        }
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a - b))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a - b))),
        _ => e_str("Cannot sub non numeric values"),
    })
}

pub fn mul<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a * b))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a * b))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a * b))),
        _ => e_str("Cannot add non numeric values"),
    })
}

pub fn div<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a / b))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a / b))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a / b))),
        _ => e_str("Cannot add non numeric values"),
    })
}

pub fn modulo<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a % b))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a % b))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a % b))),
        _ => e_str("Cannot add non numeric values"),
    })
}

pub fn min<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a.min(b)))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a.min(b)))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a.min(b)))),
        _ => e_str("Can only min numbers"),
    })
}
pub fn max<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    fold(l, |a, b| match num_match(&a, b) {
        Some(U(a, b)) => Ok(TCow::Owned(TData::UInt(a.max(b)))),
        Some(F(a, b)) => Ok(TCow::Owned(TData::Float(a.max(b)))),
        Some(I(a, b)) => Ok(TCow::Owned(TData::Int(a.max(b)))),
        _ => e_str("Can only min numbers"),
    })
}
