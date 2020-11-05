//! The section about making providing functions for the templates

use crate::*;
use func_man::*;
use std::path::PathBuf;

mod bools;
mod exec;
mod file;
mod folder;
mod format;
mod lists;
mod maps;
pub mod math;
mod random;
mod strings;
mod svg;
mod table;
mod write;

pub trait WithFuncs: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>) -> Self;
    fn with_fn<K: Into<String>>(self, k: K, f: TFn) -> Self {
        self.with_f(k, Box::new(f))
    }

    fn with_defaults(self) -> Self {
        self.with_bools()
            .with_strings()
            .with_math()
            .with_files()
            .with_lists()
            .with_maps()
            .with_format()
            .with_rand()
    }

    fn with_rand(self) -> Self {
        self.with_fn("rand", random::get_rand)
    }
    fn with_svg(self) -> Self {
        self.with_fn("xy", svg::xy)
            .with_fn("xywh", svg::xywh)
            .with_fn("xy12", svg::xy12)
            .with_fn("fl_stk", svg::fl_stk)
            .with_fn("xml_esc", svg::xml_esc)
            .with_fn("font", svg::font)
    }
    fn with_maps(self) -> Self {
        self.with_fn("map", maps::map)
    }

    fn with_format(self) -> Self {
        self.with_fn("r_json", format::r_json)
            .with_fn("r_card", format::r_card)
    }

    fn with_lists(self) -> Self {
        self.with_fn("list", lists::list)
            .with_fn("sort", lists::sort)
            .with_fn("append", lists::append)
            .with_fn("sort_on", lists::sort_on)
            .with_fn("bin_seatch", lists::bin_search)
            .with_fn("bin_get", lists::bin_get)
            .with_fn("get", lists::get)
            .with_fn("filter", lists::filter)
            .with_fn("len", lists::len)
            .with_fn("slice", lists::slice)
            .with_fn("groups", lists::groups)
    }

    fn with_strings(self) -> Self {
        self.with_fn("cat", strings::cat)
            .with_fn("md", strings::md)
            .with_fn("table", strings::table)
            .with_fn("split", strings::split)
            .with_fn("str_contains", strings::contains)
            .with_fn("str_replace", strings::replace)
            .with_fn("str_replace_n", strings::replace_n)
            .with_fn("html_esc", strings::html_esc)
            .with_fn("regex", strings::regex)
            .with_fn("word_wrap", strings::word_wrap)
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
            .with_fn("neq", bools::neq)
            .with_fn("eq_any", bools::eq_any)
            .with_fn("gt", bools::gt)
            .with_fn("gte", bools::gte)
            .with_fn("lt", bools::lt)
            .with_fn("lte", bools::lte)
            .with_fn("and", bools::and)
            .with_fn("nand", bools::nand)
            .with_fn("or", bools::or)
            .with_fn("nor", bools::nor)
    }

    fn with_files(self) -> Self {
        self.with_fn("parent", file::parent)
            .with_fn("join", file::join)
            .with_fn("bread_crumbs", file::bread_crumbs)
            .with_fn("base_name", file::base_name)
            .with_fn("base_name_sure", file::base_name_sure)
            .with_fn("with_ext", file::with_ext)
            .with_fn("stem", file::stem)
            .with_fn("full_stem", file::full_stem)
    }

    fn with_folder_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("dir", folder::dir(pb.clone()))
            .with_f("file", folder::file(pb.clone()))
            .with_f("is_file", folder::is_file(pb.clone()))
            .with_f("is_dir", folder::is_dir(pb.clone()))
            .with_f("scan_dir", folder::scan_dir(pb.clone()))
    }

    fn with_exec(self) -> Self {
        self.with_fn("exec", exec::exec)
            .with_fn("exec_stdin", exec::exec_stdin)
    }

    fn with_write_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("write", write::write(pb.clone()))
    }
}

impl<FA: FuncAdder> WithFuncs for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>) -> Self {
        self.with_func(k, f)
    }
}
