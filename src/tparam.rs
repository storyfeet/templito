use crate::*;
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::*;
use tdata::*;

use expr::VarPart;

pub type TBop<'a> = Option<TCow<'a>>;
pub type TCow<'a> = Cow<'a, TData>;
pub fn b_ok(d: TData) -> anyhow::Result<TCow<'static>> {
    Ok(Cow::Owned(d))
}

pub trait TParam: Debug {
    fn get_v<'a>(&'a self, s: &[VarPart]) -> TBop<'a>;
}

impl TParam for TData {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> TBop<'a> {
        if l.len() == 0 {
            return Some(Cow::Borrowed(self));
        }
        match (self, &l[0]) {
            (TData::Map(m), VarPart::Id(k)) => m.get(k)?.get_v(&l[1..]),
            (TData::List(m), VarPart::Num(k)) => m.get(*k)?.get_v(&l[1..]),
            _ => None,
        }
    }
}

impl TParam for toml::Value {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> TBop<'a> {
        use toml::Value as TV;
        if l.len() == 0 {
            return Some(Cow::Owned(TData::from_toml(self.clone())));
        }
        match (self, &l[0]) {
            (TV::Table(m), VarPart::Id(k)) => {
                let vp = m.get(k)?.get_v(&l[1..])?;
                Some(vp)
            }

            (TV::Array(m), VarPart::Num(k)) => {
                let vp = m.get(*k)?.get_v(&l[1..])?;
                Some(vp)
            }
            _ => None,
        }
    }
}

impl TParam for serde_json::Value {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> TBop<'a> {
        use serde_json::Value as SV;
        if l.len() == 0 {
            return Some(Cow::Owned(TData::from_json(self.clone())));
        }
        match (self, &l[0]) {
            (SV::Object(m), VarPart::Id(k)) => {
                let vp = m.get(k)?.get_v(&l[1..])?;
                Some(vp)
            }

            (SV::Array(m), VarPart::Num(k)) => {
                let vp = m.get(*k)?.get_v(&l[1..])?;
                Some(vp)
            }
            _ => None,
        }
    }
}

impl TParam for String {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::String(self.clone())))
    }
}
impl TParam for str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::String(String::from(self))))
    }
}

impl TParam for &str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::String(String::from(*self))))
    }
}
impl TParam for usize {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::UInt(*self)))
    }
}
impl TParam for f64 {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::Float(*self)))
    }
}
impl TParam for bool {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        Some(Cow::Owned(TData::Bool(*self)))
    }
}

impl<'a> TParam for TCow<'a> {
    fn get_v<'b>(&'b self, s: &[VarPart]) -> TBop<'b> {
        self.deref().get_v(s)
    }
}

impl TParam for Vec<&str> {
    fn get_v<'a>(&'a self, s: &[VarPart]) -> TBop<'a> {
        match s.get(0) {
            None => {
                return Some(Cow::Owned(TData::List(
                    self.iter().map(|s| TData::String(s.to_string())).collect(),
                )));
            }
            Some(VarPart::Num(n)) => self
                .get(*n)
                .map(|s| Cow::Owned(TData::String(s.to_string()))),
            _ => None,
        }
    }
}
