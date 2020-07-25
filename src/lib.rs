mod err;
mod parser;
mod pipeline;
mod scope;
mod template;
mod tests;
use template::{TreeTemplate, VarPart};

use std::fmt::{Debug, Display};

pub trait Templable: Sized + PartialEq + Debug + Display + Clone {
    type FErr: 'static + std::error::Error + Sync + Send;
    fn parse_lit(s: &str) -> Result<Self, Self::FErr>;
    fn as_bool(&self) -> Option<bool> {
        None
    }
    fn keys(&self) -> Option<Vec<String>> {
        None
    }
    fn len(&self) -> Option<usize> {
        None
    }
    fn get_key<'a>(&'a self, _s: &str) -> Option<&'a Self> {
        None
    }
    fn get_index<'a>(&'a self, _n: usize) -> Option<&'a Self> {
        None
    }
    fn get_func<'a>(&'a self, _s: &str) -> Option<SFunc<'a, Self>> {
        None
    }

    fn get_var_part<'a>(&'a self, v: &VarPart) -> Option<&'a Self> {
        match v {
            VarPart::Num(n) => self.get_index(*n),
            VarPart::Id(s) => self.get_key(s),
        }
    }

    fn get_var_path<'a>(&'a self, v: &[VarPart]) -> Option<&'a Self> {
        if v.len() == 0 {
            return Some(self);
        }
        self.get_var_part(&v[0])
            .and_then(|p| p.get_var_path(&v[1..]))
    }
}

pub type TFunc<'a, T: Templable> = &'a dyn Fn(&[T]) -> Result<T, T::FErr>;
pub type SFunc<'a, T: Templable> = &'a dyn Fn(&T, &[T]) -> Result<T, T::FErr>;

pub trait TempManager {
    fn insert(&mut self, k: String, t: TreeTemplate);
    fn get(&mut self, k: &str) -> Option<&TreeTemplate>;
}

pub trait FuncManager<T: Templable> {
    fn get_func(&self, k: &str) -> Option<TFunc<T>>;
}
