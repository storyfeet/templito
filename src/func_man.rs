use crate::*;
use funcs::WithFuncs;
use std::collections::HashMap;
use std::path::PathBuf;
use tparam::*;

pub type TFunc = dyn for<'a> Fn(&[TBoco<'a>]) -> anyhow::Result<TBoco<'a>>;
pub type TFn = for<'a> fn(&[TBoco<'a>]) -> anyhow::Result<TBoco<'a>>;

pub trait FuncManager {
    fn get_func(&self, k: &str) -> Option<&TFunc>;
}

pub trait FuncAdder: Sized {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc>);
    fn add_fn<K: Into<String>>(&mut self, k: K, f: TFn) {
        self.add_func(k, Box::new(f));
    }
    fn with_func<K: Into<String>>(mut self, k: K, f: Box<TFunc>) -> Self {
        self.add_func(k, f);
        self
    }
    fn with_fn<K: Into<String>>(mut self, k: K, f: TFn) -> Self {
        self.add_fn(k, f);
        self
    }
}

pub type BasicFuncs = HashMap<String, Box<TFunc>>;

impl FuncManager for BasicFuncs {
    fn get_func(&self, k: &str) -> Option<&TFunc> {
        self.get(k).map(|r| &**r)
    }
}

impl FuncAdder for BasicFuncs {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc>) {
        self.insert(k.into(), f);
    }
}

pub fn default_func_man() -> BasicFuncs {
    BasicFuncs::new().with_defaults()
}

pub fn func_man_folders<P: Into<PathBuf>>(p: P) -> BasicFuncs {
    BasicFuncs::new().with_defaults().with_folder_lock(p)
}
