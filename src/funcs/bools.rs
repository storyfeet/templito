use crate::*;
use err::*;

pub fn eq(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if *a != l[0] {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}

pub fn gt(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if l[0] > *a {
        } else {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}

pub fn lt(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lt"));
    }
    for a in &l[1..] {
        if l[0] < *a {
        } else {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}

pub fn gte(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for gte"));
    }
    for a in &l[1..] {
        if !(l[0] >= *a) {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}
pub fn lte(l: &[TData]) -> anyhow::Result<TData> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lte"));
    }
    for a in &l[1..] {
        if !(l[0] <= *a) {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}

pub fn and(l: &[TData]) -> anyhow::Result<TData> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return Ok(TData::Bool(false));
        }
    }
    Ok(TData::Bool(true))
}
pub fn or(l: &[TData]) -> anyhow::Result<TData> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return Ok(TData::Bool(true));
        }
    }
    Ok(TData::Bool(false))
}
