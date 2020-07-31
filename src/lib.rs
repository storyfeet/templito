pub mod err;
pub mod func_man;
pub mod funcs;
pub mod impls;
mod parser;
mod pipeline;
mod scope;
pub mod temp_man;
pub mod template;
mod tests;
use template::{TreeTemplate, VarPart};

use std::fmt::{Debug, Display};

pub trait Templable: 'static + Sized + PartialEq + Debug + Display + Clone {
    //type FErr: 'static + std::error::Error + Sync + Send;
    fn parse_lit(s: &str) -> anyhow::Result<Self>;
    fn string(s: &str) -> Self;
    fn as_str(&self) -> Option<&str> {
        None
    }
    fn bool(b: bool) -> Self;
    fn as_bool(&self) -> Option<bool> {
        None
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn usize(u: usize) -> Self;
    fn as_usize(&self) -> Option<usize> {
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
    fn get_func<'a>(&'a self, _s: &str) -> Option<&'a func_man::TFunc<Self>> {
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

    fn compare(&self, b: &Self) -> Option<std::cmp::Ordering>;
}
