use super::folder::safe_path;
use crate::*;
use boco::*;
use err::*;
use func_man::*;

use std::io::Write;
use std::path::PathBuf;

pub fn write(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        if l.len() < 2 {
            return Err(ea_str("write requires path and contents"));
        }

        //get safe path
        let sp = ea_op(safe_path(&pb, &l[0].to_string()), "Broken File path")?;
        //Make directory
        std::fs::create_dir_all(sp.parent().ok_or(ea_str("No Parent folder"))?)?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(sp)?;

        write!(file, "{}", l[1].to_string())?;

        b_ok(TData::String(String::new()))
    })
}
