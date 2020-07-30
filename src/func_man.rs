use crate::*;
use funcs::WithFuncs;
use std::collections::HashMap;

pub type TFunc<T> = dyn Fn(&[T]) -> anyhow::Result<T>;
pub type TFn<T> = fn(&[T]) -> anyhow::Result<T>;

pub trait FuncManager<T: Templable> {
    fn get_func(&self, k: &str) -> Option<&TFunc<T>>;
}

pub trait FuncAdder<T: 'static + Templable>: Sized {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc<T>>);
    fn add_fn<K: Into<String>>(&mut self, k: K, f: TFn<T>) {
        self.add_func(k, Box::new(f));
    }
    fn with_func<K: Into<String>>(mut self, k: K, f: Box<TFunc<T>>) -> Self {
        self.add_func(k, f);
        self
    }
    fn with_fn<K: Into<String>>(mut self, k: K, f: TFn<T>) -> Self {
        self.add_fn(k, f);
        self
    }
}

pub type BasicFuncs<T> = HashMap<String, Box<TFunc<T>>>;

impl<T: Templable> FuncManager<T> for BasicFuncs<T> {
    fn get_func(&self, k: &str) -> Option<&TFunc<T>> {
        self.get(k).map(|r| &**r)
    }
}

impl<T: 'static + Templable> FuncAdder<T> for BasicFuncs<T> {
    fn add_func<K: Into<String>>(&mut self, k: K, f: Box<TFunc<T>>) {
        self.insert(k.into(), f);
    }
}

pub fn default_func_man<T: Templable>() -> BasicFuncs<T> {
    BasicFuncs::new().with_basics()
}

//Section for Common Funcs