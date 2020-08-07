//! The section about making providing functions for the templates

use crate::*;
use func_man::*;
use std::path::PathBuf;

mod bools;
mod file;
mod folder;
mod math;
mod strings;
mod table;

pub trait WithFuncs: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>) -> Self;
    fn with_fn<K: Into<String>>(self, k: K, f: TFn) -> Self {
        self.with_f(k, Box::new(f))
    }

    fn with_defaults(self) -> Self {
        self.with_bools().with_strings().with_math().with_files()
    }

    fn with_strings(self) -> Self {
        self.with_fn("cat", strings::cat)
            .with_fn("md", strings::md)
            .with_fn("table", strings::table)
    }

    fn with_math(self) -> Self {
        self.with_fn("add", math::add)
            .with_fn("sub", math::sub)
            .with_fn("mul", math::mul)
            .with_fn("div", math::div)
            .with_fn("mod", math::modulo)
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

    fn with_files(self) -> Self {
        self.with_fn("parent", file::parent)
            .with_fn("join", file::join)
            .with_fn("bread_crumbs", file::bread_crumbs)
            .with_fn("base_name", file::base_name)
            .with_fn("base_name_sure", file::base_name_sure)
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
