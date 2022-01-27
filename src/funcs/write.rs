use super::folder::safe_path;
use crate::*;
use err_tools::*;
use func_man::*;
use tparam::*;

use std::io::Write;
use std::path::PathBuf;

pub fn write(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        if l.len() < 2 {
            return e_str("write requires path and contents");
        }

        //get safe path
        let sp = safe_path(&pb, &l[0].to_string()).e_str("Broken File path")?;
        //Make directory
        std::fs::create_dir_all(sp.parent().e_str("No Parent folder")?)?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(sp)?;

        write!(file, "{}", l[1].to_string())?;

        b_ok(TData::String(String::new()))
    })
}
