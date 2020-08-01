//! The section about making providing functions for the templates

use crate::*;
use func_man::*;
use std::path::PathBuf;

mod bools;
mod folder;
mod nums;
mod strings;

pub trait WithFuncs<T: Templable>: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self;

    fn with_defaults(self) -> Self {
        self.with_bools().with_strings()
    }

    fn with_strings(self) -> Self {
        self.with_f("cat", strings::cat())
            .with_f("md", strings::md())
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

    fn with_folder_lock<P: Into<PathBuf>>(self, pb: PathBuf) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("dir", folder::dir(pb.clone()))
            .with_f("file", folder::file(pb.clone()))
            .with_f("is_file", folder::is_file(pb.clone()))
            .with_f("is_dir", folder::is_dir(pb.clone()))
    }
}

impl<FA: FuncAdder<T>, T: Templable> WithFuncs<T> for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc<T>>) -> Self {
        self.with_func(k, f)
    }
}
