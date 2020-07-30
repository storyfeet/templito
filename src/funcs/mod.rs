//! The section about making providing functions for the templates

use crate::*;
use func_man::*;

mod bools;

pub trait WithFuncs<T: Templable>: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self;

    fn with_defaults(self) -> Self {
        self.with_bools().with_strings()
    }

    fn with_strings(self) -> Self {
        self.with_f("cat", cat())
    }

    fn with_bools(self) -> Self {
        self.with_f("eq", bools::eq())
            .with_f("gt", bools::gt())
            .with_f("gte", bools::gte())
            .with_f("lt", bools::lt())
            .with_f("lte", bools::lte())
            .with_f("and", bools::and())
            .with_f("or", bools::or())
    }
}

impl<FA: FuncAdder<T>, T: Templable> WithFuncs<T> for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self {
        self.with_func(k, f)
    }
}

fn cat<T: Templable>() -> Box<TFunc<T>> {
    Box::new(|l: &[T]| -> anyhow::Result<T> {
        let mut r_str = String::new();
        for v in l {
            match &v.as_str() {
                Some(s) => r_str.push_str(s),
                None => r_str.push_str(&v.to_string()),
            }
        }
        Ok(T::string(&r_str))
    })
}
