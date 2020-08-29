use crate::*;
use boco::*;
use err::*;
use std::path::{Path, PathBuf};
use tparam::*;

fn path_dat<'a>(p: &Path) -> TData {
    TData::String(p.display().to_string())
}

fn join_args<'a>(args: &[TBoco<'a>]) -> PathBuf {
    let mut path = PathBuf::new();
    for v in args {
        path.push(v.to_string());
    }
    path
}

pub fn parent<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if let Some(p) = join_args(args).parent() {
        b_ok(TData::String(p.display().to_string()))
    } else {
        b_ok(TData::String("".to_string()))
    }
}

pub fn join<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    b_ok(path_dat(&join_args(args)))
}

pub fn bread_crumbs<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut path = join_args(args);
    let mut res = Vec::new();
    res.push(path_dat(&path));
    while let Some(par) = path.parent() {
        res.insert(0, path_dat(&par));
        path = PathBuf::from(par);
    }
    b_ok(TData::List(res))
}

pub fn base_name<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    join_args(args)
        .file_name()
        .and_then(|v| v.to_str())
        .map(|v| TBoco::Co(TData::String(v.to_string())))
        .ok_or(ea_str("Could not get file base_name"))
}
pub fn base_name_sure<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    b_ok(TData::String(
        join_args(args)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string(),
    ))
}

pub fn with_ext<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 2 {
        return Err(ea_str("'with_ext' requires a filenae then an extension"));
    }
    let p = PathBuf::from(args[0].to_string());
    let pe = p.with_extension(args[1].to_string());
    b_ok(path_dat(&pe))
}

pub fn stem<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    join_args(args)
        .file_stem()
        .map(|s| TBoco::Co(TData::String(s.to_string_lossy().to_string())))
        .ok_or(ea_str("No File to stem").into())
}
pub fn full_stem<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let j = join_args(args);
    let stem = j.file_stem().ok_or(ea_str("No file to stem"))?;
    b_ok(TData::String(j.with_file_name(stem).display().to_string()))
}
