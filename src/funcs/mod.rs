//! The section about making providing functions for the templates

use crate::*;
use func_man::*;
use std::path::PathBuf;

mod bools;
mod folder;
mod math;
mod strings;

pub trait WithFuncs {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>) -> Self;

    fn with_defaults(self) -> Self {
        self.with_bools().with_strings()
    }

    fn with_strings(self) -> Self {
        self.with_f("cat", strings::cat())
            .with_f("md", strings::md())
    }

    fn with_bools(self) -> Self {
        self.with_fn("eq", bools::eq)
            .with_fn("gt", bools::gt)
            .with_fn("gte", bools::gte)
            .with_fn("lt", bools::lt)
            .with_fn("lte", bools::lte)
            .with_fn("and", bools::and)
            .with_fn("or", bools::or)
    }

    fn with_folder_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("dir", folder::dir(pb.clone()))
            .with_f("file", folder::file(pb.clone()))
            .with_f("is_file", folder::is_file(pb.clone()))
            .with_f("is_dir", folder::is_dir(pb.clone()))
            .with_f("join", folder::join())
    }
}

impl<FA: FuncAdder> WithFuncs for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>) -> Self {
        self.with_func(k, f)
    }
}
