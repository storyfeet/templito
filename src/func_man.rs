use crate::*;
pub use funcs::WithFuncs;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use tparam::*;

pub type TFunc = dyn for<'a> Fn(&[TCow<'a>]) -> anyhow::Result<TCow<'a>>;
pub type TFn = for<'a> fn(&[TCow<'a>]) -> anyhow::Result<TCow<'a>>;

pub trait FuncManager {
    fn get_func(&self, k: &str) -> Option<&TFunc>;
    fn for_each<F: Fn(&String, &str)>(&self, f: F);

    fn print_all(&self) {
        self.for_each(|k, v| {
            print!("\n {}:", k);
            for sp in v.split(":") {
                println!("\t\t{}", sp);
            }
        })
    }

    fn print_filter(&self, s: &str) {
        self.for_each(|k, v| {
            if !k.contains(s) && !v.contains(s) {
                return;
            }
            print!("\n {}:", k);
            for sp in v.split(":") {
                println!("\t\t{}", sp);
            }
        })
    }
}

pub trait FuncAdder: Sized {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc>, description: &'static str);
    fn add_fn<K: Into<String>>(&mut self, k: K, f: TFn, d: &'static str) {
        self.add_func(k, Box::new(f), d);
    }
    fn with_func<K: Into<String>>(mut self, k: K, f: Box<TFunc>, d: &'static str) -> Self {
        self.add_func(k, f, d);
        self
    }
    fn with_fn<K: Into<String>>(mut self, k: K, f: TFn, d: &'static str) -> Self {
        self.add_fn(k, f, d);
        self
    }
}

pub struct BasicFuncs {
    funcs: HashMap<String, Box<TFunc>>,
    descriptions: BTreeMap<String, &'static str>,
}

impl BasicFuncs {
    pub fn new() -> Self {
        BasicFuncs {
            funcs: HashMap::new(),
            descriptions: BTreeMap::new(),
        }
    }
}

impl FuncManager for BasicFuncs {
    fn get_func(&self, k: &str) -> Option<&TFunc> {
        self.funcs.get(k).map(|r| &**r)
    }
    fn for_each<F: Fn(&String, &str)>(&self, f: F) {
        for (k, &v) in &self.descriptions {
            f(k, v)
        }
    }
}

impl FuncAdder for BasicFuncs {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc>, description: &'static str) {
        let ks: String = k.into();
        self.funcs.insert(ks.clone(), f);
        self.descriptions.insert(ks, description);
    }
}

pub fn default_func_man() -> BasicFuncs {
    BasicFuncs::new().with_defaults()
}

pub fn func_man_folders<P: Into<PathBuf>>(p: P) -> BasicFuncs {
    BasicFuncs::new().with_defaults().with_folder_lock(p)
}
