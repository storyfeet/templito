use crate::*;
use std::fmt::Debug;
use std::ops::*;

use boco::*;

pub type TBop<'a> = Bop<'a, TData>;
pub type TBoco<'a> = Boco<'a, TData>;

pub trait TParam: Debug {
    fn get_v<'a>(&'a self, s: &[VarPart]) -> TBop<'a>;
}

impl TParam for TData {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> TBop<'a> {
        if l.len() == 0 {
            return boco_b(self);
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
            return boco_c(TData::from_toml(self.clone()));
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
            return boco_c(TData::from_json(self.clone()));
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
        boco_c(TData::String(self.clone()))
    }
}
impl TParam for str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        boco_c(TData::String(String::from(self)))
    }
}

impl TParam for &str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        boco_c(TData::String(String::from(*self)))
    }
}
impl TParam for usize {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        boco_c(TData::UInt(*self))
    }
}
impl TParam for f64 {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        boco_c(TData::Float(*self))
    }
}
impl TParam for bool {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> TBop<'a> {
        boco_c(TData::Bool(*self))
    }
}

impl<'a> TParam for TBoco<'a> {
    fn get_v<'b>(&'b self, s: &[VarPart]) -> TBop<'b> {
        self.deref().get_v(s)
    }
}
