use crate::*;
use err_tools::*;
use std::ops::Deref;
use tparam::*;

pub fn eq<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for eq");
    }
    for a in &l[1..] {
        if *a != l[0] {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}
pub fn eq_any<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for eq");
    }
    for a in &l[1..] {
        if *a == l[0] {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}
pub fn neq<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for neq");
    }
    for a in &l[1..] {
        if *a != l[0] {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}

pub fn gt<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for eq");
    }
    for a in &l[1..] {
        if l[0] > *a {
        } else {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn lt<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for lt");
    }
    for a in &l[1..] {
        if l[0] < *a {
        } else {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn gte<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for gte");
    }
    for a in &l[1..] {
        if !(l[0] >= *a) {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}
pub fn lte<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Not enough args for lte");
    }
    for a in &l[1..] {
        if !(l[0] <= *a) {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn and<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

pub fn nand<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    for a in l {
        if let None | Some(false) = a.as_bool() {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}
pub fn or<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}

pub fn nor<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    for a in l {
        if let Some(true) = a.as_bool() {
            return b_ok(TData::Bool(false));
        }
    }
    b_ok(TData::Bool(true))
}

fn _type_of(td: &TData) -> &'static str {
    match td {
        TData::Bool(_) => "bool",
        TData::String(_) => "string",
        TData::Int(_) => "int",
        TData::UInt(_) => "uint",
        TData::Float(_) => "float",
        TData::List(_) => "list",
        TData::Map(_) => "map",
        TData::Template(_) => "template",
        TData::Null => "null",
        TData::Date(_) => "date",
        TData::Bytes(_) => "bytes",
    }
}

pub fn is_null<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Is WHAT null? I need a parameter");
    }

    return b_ok(TData::Bool(l[0].deref().eq(&TData::Null)));
}
pub fn is_num<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if l.len() == 0 {
        return e_str("Is WHAT num? I need a parameter");
    }
    match l[0].deref() {
        TData::UInt(_) | TData::Int(_) | TData::Float(_) => return b_ok(TData::Bool(true)),
        _ => return b_ok(TData::Bool(false)),
    }
}
pub fn type_of<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let r = _type_of(
        l.get(0)
            .e_str("missing params : type_of <item> <?match>")?
            .deref(),
    );
    match l.get(1) {
        Some(b) => b_ok(TData::Bool(r == b.to_string())),
        None => b_ok(TData::String(r.to_string())),
    }
}
