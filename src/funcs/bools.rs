use crate::*;
use err::*;
use std::cmp::Ordering::*;

pub fn eq(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if *a != l[0] {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}

pub fn gt(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if let Some(Greater) = l[0].compare(a) {
        } else {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}

pub fn lt(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lt"));
    }
    for a in &l[1..] {
        if let Some(Less) = l[0].compare(a) {
        } else {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}

pub fn gte(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for gte"));
    }
    for a in &l[1..] {
        if let None | Some(Less) = l[0].compare(a) {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}
pub fn lte(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lte"));
    }
    for a in &l[1..] {
        if let None | Some(Greater) = l[0].compare(a) {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}

pub fn and(l: &[TData]) -> anyhow::Result<TData> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return Ok(TData::bool(false));
        }
    }
    Ok(TData::bool(true))
}
pub fn or(l: &[TData]) -> anyhow::Result<TData> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return Ok(TData::bool(true));
        }
    }
    Ok(TData::bool(false))
}
