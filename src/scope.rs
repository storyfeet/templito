use crate::*;
use std::collections::HashMap;
use template::VarPart;
use tparam::*;

#[derive(Debug)]
pub struct Scope<'a> {
    params: &'a [&'a dyn TParam],
    maps: Vec<HashMap<String, TData>>,
}

impl<'a> Scope<'a> {
    pub fn new(params: &'a [&'a dyn TParam]) -> Self {
        Scope {
            params,
            maps: vec![HashMap::new()],
        }
    }

    pub fn top(self) -> HashMap<String, TData> {
        self.maps.into_iter().next().unwrap_or(HashMap::new())
    }

    pub fn get<'b>(&'b self, v: &[VarPart]) -> Bop<'b> {
        if v.len() == 0 {
            return None;
        }
        match &v[0] {
            VarPart::Num(n) => self.params.get(*n)?.get_v(&v[1..]),
            VarPart::Id(s) => {
                for i in 0..self.maps.len() {
                    let vpos = self.maps.len() - 1 - i;
                    if let Some(base) = self.maps[vpos].get(s) {
                        return base.get_v(&v[1..]);
                    }
                }
                None
            }
        }
    }

    pub fn set<K: Into<String>>(&mut self, k: K, v: TData) {
        assert!(self.maps.len() > 0);
        let p = self.maps.len() - 1;
        self.maps[p].insert(k.into(), v);
    }

    pub fn set_root<K: Into<String>>(&mut self, k: K, v: TData) {
        assert!(self.maps.len() > 0);
        self.maps[0].insert(k.into(), v);
    }

    pub fn push(&mut self) {
        self.maps.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        if self.maps.len() > 1 {
            self.maps.pop();
        }
    }
}
