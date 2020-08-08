use crate::err::*;
use crate::TData;
use std::path::{Path, PathBuf};

fn path_dat(p: &Path) -> TData {
    TData::String(p.display().to_string())
}

fn join_args(args: &[TData]) -> PathBuf {
    let mut path = PathBuf::new();
    for v in args {
        path.push(v.to_string());
    }
    path
}

pub fn parent(args: &[TData]) -> anyhow::Result<TData> {
    if let Some(p) = join_args(args).parent() {
        Ok(TData::String(p.display().to_string()))
    } else {
        Ok(TData::String("".to_string()))
    }
}

pub fn join(args: &[TData]) -> anyhow::Result<TData> {
    Ok(path_dat(&join_args(args)))
}

pub fn bread_crumbs(args: &[TData]) -> anyhow::Result<TData> {
    let mut path = join_args(args);
    let mut res = Vec::new();
    res.push(path_dat(&path));
    while let Some(par) = path.parent() {
        res.insert(0, path_dat(&par));
        path = PathBuf::from(par);
    }
    Ok(TData::List(res))
}

pub fn base_name(args: &[TData]) -> anyhow::Result<TData> {
    join_args(args)
        .file_name()
        .and_then(|v| v.to_str())
        .map(|v| TData::String(v.to_string()))
        .ok_or(ea_str("Could not get file base_name"))
}
pub fn base_name_sure(args: &[TData]) -> anyhow::Result<TData> {
    Ok(TData::String(
        join_args(args)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string(),
    ))
}

pub fn with_ext(args: &[TData]) -> anyhow::Result<TData> {
    if args.len() < 2 {
        return Err(ea_str("'with_ext' requires a filenae then an extension"));
    }
    let p = PathBuf::from(args[0].to_string());
    let pe = p.with_extension(args[1].to_string());
    Ok(path_dat(&pe))
}
