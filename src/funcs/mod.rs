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
        self.with_fn(
            "as_base64",
            bytes::as_base64,
            "(int)->string: Convert a number to base 64",
        )
        .with_fn(
            "from_base64",
            bytes::from_base64,
            "(string)->int:Convert a to a number from base 64 string ",
        )
    }

    fn with_rand(self) -> Self {
        self.with_fn(
            "rand",
            random::get_rand,
            "(?min,max)->int|(list)->value : Get a random number or item from list",
        )
    }
    fn with_svg(self) -> Self {
        self.with_fn(
            "xy",
            svg::xy,
            "(x,y,?units)->string : set svg x and y values",
        )
        .with_fn(
            "xywh",
            svg::xywh,
            "(x,y,w,h,?units)->string : set svg x,y,width and hight values",
        )
        .with_fn(
            "xy12",
            svg::xy12,
            "(x,y,x2,y2,?units)->string : set svg x,y,x2 and y2 values",
        )
        .with_fn(
            "fl_stk",
            svg::fl_stk,
            "(fill_color,stroke_color,stroke_width)->string : set svg fill and stroke values",
        )
        .with_fn(
            "xml_esc",
            svg::xml_esc,
            "(string)->string : escape an xml string",
        )
        .with_fn("font", svg::font, "(string)->string : set svg font")
    }
    fn with_maps(self) -> Self {
        self.with_fn(
            "map",
            maps::map,
            "(key,value,key,value, ...)->map : build map from list of key value pairs",
        )
    }

    fn with_format(self) -> Self {
        self.with_fn(
            "r_json",
            format::r_json,
            "(string)->value : read json data to value, normally a map",
        )
        .with_fn(
            "r_card",
            format::r_card,
            "(string)->map : read card_format data to list of values",
        )
    }

    fn with_lists(self) -> Self {
        self.with_fn(
            "list",
            lists::list,
            "(arg ...)->list<arg> : convert args to a single list",
        )
        .with_fn(
            "sort",
            lists::sort,
            "(list ...)->list : combine lists and sort final list on simple value",
        )
        .with_fn(
            "append",
            lists::append,
            "(list ...)->list : join all list args to single list",
        )
        .with_fn(
            "sort_on",
            lists::sort_on,
            "(list, criteria...)->list : sort list by criteria",
        )
        .with_fn(
            "bin_search",
            lists::bin_search,
            "(list, criteria)->index : search sorted list for comparator must match sort criteria",
        )
        .with_fn(
            "bin_get",
            lists::bin_get,
            "(list, criteria)->value : search sorted list for comparator must match sort criteria",
        )
        .with_fn(
            "get",
            lists::get,
            "(list,index)->value : get item from list by index (list,index)->value",
        )
        .with_fn(
            "filter",
            lists::filter,
            "filter list by criteria (list,criteria)->list",
        )
        .with_fn("len", lists::len, "(list)->int : Length of the list")
        .with_fn(
            "slice",
            lists::slice,
            "(list|string, start,?end)->list|string : take a section of a list",
        )
        .with_fn(
            "groups",
            lists::groups,
            "(int,list)->list<list> : break a list into groups of the given size",
        )
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
        self.with_fn(
            "dir",
            free_file::dir,
            "(path)->list : List the contents of a directory",
        )
        .with_fn(
            "file",
            free_file::file,
            "(path)->string : The contents of a text file",
        )
        .with_fn(
            "file_bytes",
            free_file::file_bytes,
            "(path)->bytes : The byte contents of a file",
        )
        .with_fn(
            "is_file",
            free_file::is_file,
            "(path)->bool : Dows the path point to a file",
        )
        .with_fn(
            "is_dir",
            free_file::is_dir,
            "(path)->bool : Does the path point to a Directory",
        )
        .with_fn(
            "scan_dir",
            free_file::scan_dir,
            "(path)->list : recursive list of folder ",
        )
        .with_fn(
            "file_img_dimensions",
            free_file::file_img_dimensions,
            "(path)->map<w,h> : get width and height of an image file without reading the whole file",
        )
        .with_fn("write", free_file::write ,"(path,contents ...)->bool : Write the contents to the file",)
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
