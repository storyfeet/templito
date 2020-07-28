use crate::*;
use std::collections::HashMap;
use template::VarPart;

pub struct Scope<'a, T: Templable> {
    params: &'a [T],
    maps: Vec<HashMap<String, T>>,
}

impl<'a, T: Templable> Scope<'a, T> {
    pub fn new(params: &'a [T]) -> Self {
        Scope {
            params,
            maps: vec![HashMap::new()],
        }
    }
    fn get_base(&'a self, vp: &VarPart) -> Option<&'a T> {
        match vp {
            VarPart::Num(n) => self.params.get(*n),
            VarPart::Id(s) => {
                for x in 1..=self.maps.len() {
                    if let Some(v) = self.maps[self.maps.len() - x].get(s) {
                        return Some(v);
                    }
                }
                None
            }
        }
    }

    pub fn get<'b>(&'b self, v: &[VarPart]) -> Option<&'b T> {
        if v.len() == 0 {
            return None;
        }
        self.get_base(&v[0]).and_then(|r| r.get_var_path(&v[1..]))
    }

    pub fn set<K: Into<String>>(&mut self, k: K, v: T) {
        assert!(self.maps.len() > 0);
        let p = self.maps.len() - 1;
        self.maps[p].insert(k.into(), v);
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
