use crate::*;
use std::collections::HashMap;
use template::VarPart;
use tparam::TParam;

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

    pub fn param(&self, n: usize, path: &str) -> Option<TData> {
        if n >= self.params.len() {
            return None;
        }
        (*self.params[n]).get_s(path)
    }

    fn get_base(&'a self, vp: &VarPart) -> Option<TData> {
        match vp {
            VarPart::Num(n) => self.params[0].get_s(""),
            VarPart::Id(s) => {
                for x in 1..=self.maps.len() {
                    if let Some(v) = self.maps[self.maps.len() - x].get(s) {
                        return Some(v.clone());
                    }
                }
                None
            }
        }
    }

    pub fn get<'b>(&'b self, v: &[VarPart]) -> Option<&'b TData> {
        if v.len() == 0 {
            return None;
        }
        self.get_base(&v[0]).and_then(|r| r.get_var_path(&v[1..]))
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
