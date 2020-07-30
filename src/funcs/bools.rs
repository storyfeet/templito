use crate::*;
use err::*;
use func_man::*;
use std::cmp::Ordering::*;

pub fn eq<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for eq"));
        }
        for a in &l[1..] {
            if *a != l[0] {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}

pub fn gt<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for eq"));
        }
        for a in &l[1..] {
            if let Some(Greater) = l[0].compare(a) {
            } else {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}
pub fn lt<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for lt"));
        }
        for a in &l[1..] {
            if let Some(Less) = l[0].compare(a) {
            } else {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}

pub fn gte<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for gte"));
        }
        for a in &l[1..] {
            if let None | Some(Less) = l[0].compare(a) {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}
pub fn lte<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for lte"));
        }
        for a in &l[1..] {
            if let None | Some(Greater) = l[0].compare(a) {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}

pub fn and<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        for a in l {
            if let None | Some(false) = a.as_bool() {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}
pub fn or<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        for a in l {
            if let Some(true) = a.as_bool() {
                return Ok(T::bool(true));
            }
        }
        Ok(T::bool(false))
    })
}
