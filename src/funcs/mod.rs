//! The section about making providing functions for the templates

use crate::*;
use err::*;
use func_man::*;

pub trait WithFuncs<T: Templable>: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self;

    fn with_basics(self) -> Self {
        self.with_f("cat", cat()).with_f("eq", eq())
    }
}

impl<FA: FuncAdder<T>, T: Templable> WithFuncs<T> for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self {
        self.with_func(k, f)
    }
}

fn cat<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l: &[T]| -> anyhow::Result<T> {
        let mut r_str = String::new();
        for v in l {
            match &v.as_str() {
                Some(s) => r_str.push_str(s),
                None => r_str.push_str(&v.to_string()),
            }
        }
        Ok(T::string(&r_str))
    })
}

fn eq<T: Templable>() -> Box<TFunc<T>> {
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

fn gt<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l| {
        if l.len() == 0 {
            return Err(ea_str("Not enough args for eq"));
        }
        for a in &l[1..] {
            if !(l[0] > a) {
                return Ok(T::bool(false));
            }
        }
        Ok(T::bool(true))
    })
}
