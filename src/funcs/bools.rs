use crate::*;
use boco::*;
use err::*;
//use std::ops::Deref;
use tparam::*;

pub fn eq<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if *a != l[0] {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}
pub fn neq<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for neq"));
    }
    for a in &l[1..] {
        if *a != l[0] {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}

pub fn gt<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for eq"));
    }
    for a in &l[1..] {
        if l[0] > *a {
        } else {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn lt<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lt"));
    }
    for a in &l[1..] {
        if l[0] < *a {
        } else {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn gte<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for gte"));
    }
    for a in &l[1..] {
        if !(l[0] >= *a) {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}
pub fn lte<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return Err(ea_str("Not enough args for lte"));
    }
    for a in &l[1..] {
        if !(l[0] <= *a) {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn and<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn nand<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}
pub fn or<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}

pub fn nor<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}
