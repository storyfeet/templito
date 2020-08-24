use crate::*;
use std::fmt::Debug;

pub trait TParam: Debug {
    fn get_v<'a>(&'a self, s: &[VarPart]) -> Option<TData>;
    fn get_b<'a>(&'a self, s: &[VarPart]) -> Option<TData>;
}

impl TParam for TData {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> Option<TData> {
        if l.len() == 0 {
            return Some(self.clone());
        }
        match (self, &l[0]) {
            (TData::Map(m), VarPart::Id(k)) => m.get(k)?.get_v(&l[1..]).map(|c| c.clone()),
            (TData::List(m), VarPart::Num(k)) => m.get(*k)?.get_v(&l[1..]).map(|c| c.clone()),
            _ => None,
        }
    }

    fn get_b<'a>(&'a self, l: &[VarPart]) -> Option<&'a TData> {
        if l.len() == 0 {
            return Some(self.clone());
        }
        match (self, &l[0]) {
            (TData::Map(m), VarPart::Id(k)) => m.get(k)?.get_v(&l[1..]).map(|c| c.clone()),
            (TData::List(m), VarPart::Num(k)) => m.get(*k)?.get_v(&l[1..]).map(|c| c.clone()),
            _ => None,
        }
    }
}

impl TParam for toml::Value {
    fn get_v<'a>(&'a self, l: &[VarPart]) -> Option<TData> {
        use toml::Value as TV;
        if l.len() == 0 {
            return Some(TData::from_toml(self.clone()));
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
    fn get_v<'a>(&'a self, l: &[VarPart]) -> Option<TData> {
        use serde_json::Value as SV;
        if l.len() == 0 {
            return Some(TData::from_json(self.clone()));
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
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::String(self.clone()))
    }
}
impl TParam for str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::String(String::from(self)))
    }
}
impl TParam for &str {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::String(String::from(*self)))
    }
}
impl TParam for usize {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::UInt(*self))
    }
}
impl TParam for f64 {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::Float(*self))
    }
}
impl TParam for bool {
    fn get_v<'a>(&'a self, _s: &[VarPart]) -> Option<TData> {
        Some(TData::Bool(*self))
    }
}
