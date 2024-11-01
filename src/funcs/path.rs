use crate::*;
use err_tools::*;
use iter_tools::*;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use tparam::*;

fn path_dat<'a>(p: &Path) -> TData {
    TData::String(p.display().to_string())
}

fn join_args<'a>(args: &[TCow<'a>]) -> PathBuf {
    let mut path = PathBuf::new();
    for v in args {
        path.push(v.to_string());
    }
    path
}

pub fn parent<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if let Some(p) = join_args(args).parent() {
        b_ok(TData::String(p.display().to_string()))
    } else {
        b_ok(TData::String("".to_string()))
    }
}

pub fn join<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    b_ok(path_dat(&join_args(args)))
}

pub fn join_with<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 2 {
        return e_str("Expect first arg to be string, second to be list of string");
    }

    match args[1].deref() {
        TData::List(l) => b_ok(TData::String(
            l.iter()
                .map(TData::to_string)
                .join(&args[0].deref().to_string()),
        )),
        _ => e_str("Second arg not a list"),
    }
}

pub fn bread_crumbs<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let mut path = join_args(args);
    let mut res = Vec::new();
    res.push(path_dat(&path));
    while let Some(par) = path.parent() {
        res.insert(0, path_dat(&par));
        path = PathBuf::from(par);
    }
    b_ok(TData::List(res))
}

pub fn base_name<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    join_args(args)
        .file_name()
        .and_then(|v| v.to_str())
        .map(|v| TCow::Owned(TData::String(v.to_string())))
        .e_str("Could not get file base_name")
}
pub fn base_name_sure<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    b_ok(TData::String(
        join_args(args)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string(),
    ))
}

pub fn with_ext<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 2 {
        return e_str("'with_ext' requires a filenae then an extension");
    }
    let p = PathBuf::from(args[0].to_string());
    let pe = p.with_extension(args[1].to_string());
    b_ok(path_dat(&pe))
}

pub fn stem<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    join_args(args)
        .file_stem()
        .map(|s| TCow::Owned(TData::String(s.to_string_lossy().to_string())))
        .e_str("No File to stem")
}
pub fn full_stem<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let j = join_args(args);
    let stem = j.file_stem().e_str("No file to stem")?;
    b_ok(TData::String(j.with_file_name(stem).display().to_string()))
}
