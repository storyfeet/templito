use crate::*;

pub trait TParam {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData>;
    fn get_u<'a>(&'a self, u: usize) -> Option<TData>;
}

impl TParam for TData {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        if s == "" {
            return Some(self.clone());
        }
        //TODO
        unimplemented! {}
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        unimplemented! {}
        //TODO
    }
}

impl TParam for serde_json::Value {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        unimplemented! {}
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        unimplemented! {}
    }
}

impl TParam for String {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        Some(TData::String(self.clone()))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::String(self.clone()))
    }
}
impl TParam for str {
    fn get_s<'a>(&'a self, _: &str) -> Option<TData> {
        Some(TData::String(String::from(self)))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::String(String::from(self)))
    }
}
impl TParam for &str {
    fn get_s<'a>(&'a self, _: &str) -> Option<TData> {
        Some(TData::String(String::from(*self)))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::String(String::from(*self)))
    }
}
impl TParam for usize {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        Some(TData::UInt(*self))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::UInt(*self))
    }
}
impl TParam for f64 {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        Some(TData::Float(*self))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::Float(*self))
    }
}
impl TParam for bool {
    fn get_s<'a>(&'a self, s: &str) -> Option<TData> {
        Some(TData::Bool(*self))
    }
    fn get_u<'a>(&'a self, u: usize) -> Option<TData> {
        Some(TData::Bool(*self))
    }
}
