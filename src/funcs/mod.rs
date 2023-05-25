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
            "cxyrr",
            svg::cxyrr,
            "(cx,cy,rx,ry,?units)->string : set svg ellipse values",
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
        .with_fn(
            "font",
            svg::font,
            "(size,?name,?units)->string : set svg font",
        )
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
        .with_fn(
            "w_json",
            format::w_json,
            "(value)->string : json output of data",
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
            "(list, criteria...)->list : sort list by *criteria*",
        )
        .with_fn(
            "bin_search",
            lists::bin_search,
            "(list, criteria)->index : search sorted list for comparator must match sort *criteria*",
        )
        .with_fn(
            "bin_get",
            lists::bin_get,
            "(list, criteria)->value : search sorted list for comparator must match sort *criteria*",
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
        self.with_fn(
            "cat",
            strings::cat,
            "(string...)->string : Concatenate strings",
        )
        .with_fn(
            "md",
            strings::md,
            "(string)->string process a string as markdown",
        )
        .with_fn(
            "table",
            strings::table,
            "use *table notation* to create an html table",
        )
        .with_fn(
            "split",
            strings::split,
            "(string,splitter)->list<string> : split string on splitter",
        )
        .with_fn(
            "str_contains",
            strings::contains,
            "(haystack,needle...)->bool, does the haystack contain any of the needles",
        )
        .with_fn(
            "str_replace",
            strings::replace,
            "(string,needle,replacement)->string : Replace all copies of needle with replacement",
        )
        .with_fn("str_replace_n", strings::replace_n,"(string, needle,replacement,?n)->string : Replace n or 1 copies of needle with replacement")
        .with_fn("html_esc", strings::html_esc,"(string)->string : sanitize input against html security")
        .with_fn("regex", strings::regex,"(string,regex)->bool : Does the string match the regex")
        .with_fn("word_wrap", strings::word_wrap,"(string,width)->string : Add new lines to make sure text lines don't exceed width")
        .with_fn("debug", strings::debug,"(value ...)->() : print something to the log")
    }

    fn with_math(self) -> Self {
        self.with_fn(
            "add",
            math::add,
            "(number...)->number : Sum all the numbers",
        )
        .with_fn(
            "sub",
            math::sub,
            "(up,down...)->number : Subtract all downs from up",
        )
        .with_fn(
            "mul",
            math::mul,
            "(number...)->number : Multiply all the numbers",
        )
        .with_fn("div", math::div,"(top,bottom)->number : Divide the top by the bottom, if both are integers then result will be an integer")
        .with_fn("mod", math::modulo,"(top,bottom...)->int : Top Modulo Bottom")
        .with_fn("min", math::min,"(number...)->number : The lowest of the numbers")
        .with_fn("max", math::max,"(number...)->number : The highest of the numbers")
    }

    fn with_bools(self) -> Self {
        self.with_fn("eq", bools::eq, "(A,B...)->bool : Does A equal all the B's")
            .with_fn(
                "neq",
                bools::neq,
                "(A,B...)->bool : Is A different to all the B's",
            )
            .with_fn(
                "eq_any",
                bools::eq_any,
                "(A,B...)->bool : Does A equal any of the B's",
            )
            .with_fn(
                "gt",
                bools::gt,
                "(A,B...)->bool : Is A greater than all the B's",
            )
            .with_fn(
                "gte",
                bools::gte,
                "(A,B...)->bool : Is A greater than or equal to all the B's",
            )
            .with_fn(
                "lt",
                bools::lt,
                "(A,B...)->bool : Is a Less than all the B's",
            )
            .with_fn(
                "lte",
                bools::lte,
                "(A,B...)->bool : Is a Less than or equal to all the B's",
            )
            .with_fn("and", bools::and, "(A...)->bool : Are all the values true")
            .with_fn(
                "nand",
                bools::nand,
                "(A...)->bool : False if all values are true, with one are, equivilent to not",
            )
            .with_fn(
                "or",
                bools::or,
                "(A...)->bool : True if any values are true",
            )
            .with_fn(
                "nor",
                bools::nor,
                "(A...)->bool : False if any values are true",
            )
            .with_fn(
                "type_of",
                bools::type_of,
                "(value)->string : The type of the value",
            )
    }

    fn with_path(self) -> Self {
        self.with_fn("parent", path::parent, "(path)->path : The parent path")
            .with_fn("join", path::join, "(path...)->path : Join paths")
            .with_fn(
                "bread_crumbs",
                path::bread_crumbs,
                "(path)->list<path> : A list of paths for each parent of this path",
            )
            .with_fn(
                "base_name",
                path::base_name,
                "(path)->string : Return file name within the folder Panics on empty",
            )
            .with_fn(
                "base_name_sure",
                path::base_name_sure,
                "(path)->string : Return file name within the folder empty string on empty",
            )
            .with_fn(
                "with_ext",
                path::with_ext,
                "(path,ext)->path : replaces the current exstension an extension to a path,ext should not have a dot ",
            )
            .with_fn("stem", path::stem,"(path)->string : basename without extension")
            .with_fn("full_stem", path::full_stem,"(path)->path : file path without extension")
    }

    fn with_folder_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f(
            "dir",
            folder::dir(pb.clone()),
            "(path)->list<string> : Locked to folder : List files in folder",
        )
        .with_f(
            "file",
            folder::file(pb.clone()),
            "(path)->string : Locked to folder : return a files string",
        )
        .with_f(
            "file_bytes",
            folder::file_bytes(pb.clone()),
            "(path)->bytes : Locked to folder : read file as bytes",
        )
        .with_f(
            "is_file",
            folder::is_file(pb.clone()),
            "(path)->bool : Locked to folder : Does path point to file",
        )
        .with_f(
            "is_dir",
            folder::is_dir(pb.clone()),
            "(path)->bool : Locked to folder : Does path point to directory",
        )
        .with_f(
            "scan_dir",
            folder::scan_dir(pb.clone()),
            "(path)->list<map> : Locked to folder :recursively scan directory",
        )
        .with_f(
            "file_img_dimensions",
            folder::file_img_dimensions(pb.clone()),
            "(path)->map<x,y> : Locked to folder : Read img file dimensions without reading whole file"

            ,
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
        self.with_fn(
            "exec",
            exec::exec,
            "(command,arg...)->string : Run the command and return the string result",
        )
        .with_fn(
            "exec_stdin",
            exec::exec_stdin,
            "(command,stdin,arg...)->string : Run the command with stdin piped to it's std in",
        )
        .with_fn(
            "env",
            exec::env,
            "(var_name,default)->string : read environment variable, if not found, use default",
        )
    }

    fn with_write_lock<P: Into<PathBuf>>(self, pb: P) -> Self {
        let pb: PathBuf = pb.into();
        self.with_f(
            "write",
            write::write(pb.clone()),
            "(rel_path,contents)->() : write locked within specific folder",
        )
    }
}

impl<FA: FuncAdder> WithFuncs for FA {
    fn with_f<K: Into<String>>(self, k: K, f: Box<TFunc>, d: &'static str) -> Self {
        self.with_func(k, f, d)
    }
}
