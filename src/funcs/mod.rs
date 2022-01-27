//! The section about making providing functions for the templates

use crate::*;
use func_man::*;
use std::path::PathBuf;

mod bools;
mod bytes;
mod exec;
mod folder;
mod format;
mod free_file;
mod lists;
mod maps;
pub mod math;
mod path;
mod random;
mod strings;
mod svg;
mod table;
mod write;

pub trait WithFuncs: Sized {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>, d: &'static str) -> Self;
    fn with_fn<K: Into<String>>(self, k: K, f: TFn, d: &'static str) -> Self {
        self.with_f(k, Box::new(f), d)
    }

    fn with_defaults(self) -> Self {
        self.with_bools()
            .with_strings()
            .with_math()
            .with_path()
            .with_lists()
            .with_maps()
            .with_format()
            .with_rand()
            .with_bytes()
            .with_svg()
    }

    fn with_bytes(self) -> Self {
        self.with_fn("as_base64", bytes::as_base64, "Convert a number to base 64")
            .with_fn(
                "from_base64",
                bytes::from_base64,
                "Convert a to a number from base 64 string",
            )
    }

    fn with_rand(self) -> Self {
        self.with_fn(
            "rand",
            random::get_rand,
            "Get a random number or item from list",
        )
    }
    fn with_svg(self) -> Self {
        self.with_fn("xy", svg::xy, "set svg x and y values")
            .with_fn("xywh", svg::xywh, "set svg x,y,width and hight values")
            .with_fn("xy12", svg::xy12, "set svg x,y,x2 and y2 values")
            .with_fn("fl_stk", svg::fl_stk, "set svg fill and stroke values")
            .with_fn("xml_esc", svg::xml_esc, "escape an xml string")
            .with_fn("font", svg::font, "set svg font")
    }
    fn with_maps(self) -> Self {
        self.with_fn("map", maps::map, "build map from list of key then value")
    }

    fn with_format(self) -> Self {
        self.with_fn("r_json", format::r_json, "read json string data to value")
            .with_fn(
                "r_card",
                format::r_card,
                "read card_format data to list of values",
            )
    }

    fn with_lists(self) -> Self {
        self.with_fn("list", lists::list, "convert args to a single list")
            .with_fn("sort", lists::sort, "sort list on simple value")
            .with_fn("append", lists::append, "join all list args to single list")
            .with_fn(
                "sort_on",
                lists::sort_on,
                "sort list by criteria (list,criteria ...)->list",
            )
            .with_fn(
                "bin_search",
                lists::bin_search,
                "search sorted list for comparator (list,criteria)->location",
            )
            .with_fn(
                "bin_get",
                lists::bin_get,
                "get item from list by criteria (list,criteria)->value ",
            )
            .with_fn(
                "get",
                lists::get,
                "get item from list by index (list,index)->value",
            )
            .with_fn(
                "filter",
                lists::filter,
                "filter list by criteria (list,criteria)->list",
            )
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
            .with_fn("debug", strings::debug)
    }

    fn with_math(self) -> Self {
        self.with_fn("add", math::add)
            .with_fn("sub", math::sub)
            .with_fn("mul", math::mul)
            .with_fn("div", math::div)
            .with_fn("mod", math::modulo)
            .with_fn("min", math::min)
            .with_fn("max", math::max)
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
            .with_fn("type_of", bools::type_of)
    }

    fn with_path(self) -> Self {
        self.with_fn("parent", path::parent)
            .with_fn("join", path::join)
            .with_fn("bread_crumbs", path::bread_crumbs)
            .with_fn("base_name", path::base_name)
            .with_fn("base_name_sure", path::base_name_sure)
            .with_fn("with_ext", path::with_ext)
            .with_fn("stem", path::stem)
            .with_fn("full_stem", path::full_stem)
    }

    fn with_folder_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("dir", folder::dir(pb.clone()))
            .with_f("file", folder::file(pb.clone()))
            .with_f("file_bytes", folder::file_bytes(pb.clone()))
            .with_f("is_file", folder::is_file(pb.clone()))
            .with_f("is_dir", folder::is_dir(pb.clone()))
            .with_f("scan_dir", folder::scan_dir(pb.clone()))
            .with_f(
                "file_img_dimensions",
                folder::file_img_dimensions(pb.clone()),
            )
    }

    fn with_free_files(self) -> Self {
        self.with_fn("dir", free_file::dir)
            .with_fn("file", free_file::file)
            .with_fn("file_bytes", free_file::file_bytes)
            .with_fn("is_file", free_file::is_file)
            .with_fn("is_dir", free_file::is_dir)
            .with_fn("scan_dir", free_file::scan_dir)
            .with_fn("file_img_dimensions", free_file::file_img_dimensions)
            .with_fn("write", free_file::write)
    }

    fn with_exec(self) -> Self {
        self.with_fn("exec", exec::exec)
            .with_fn("exec_stdin", exec::exec_stdin)
            .with_fn("env", exec::env)
            .with_fn("env_maybe", exec::env_maybe)
    }

    fn with_write_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f("write", write::write(pb.clone()))
    }
}

impl<FA: FuncAdder> WithFuncs for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>, d: &'static str) -> Self {
        self.with_func(k, f, d)
    }
}
