use super::WithFuncs;
use crate::*;
use boco::*;
use err::*;
use func_man::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tparam::*;

fn safe_path(p: &Path, s: &str) -> Option<PathBuf> {
    let pres = p.join(s);
    if pres.starts_with(p) {
        return Some(pres);
    }
    None
}

fn do_dir<'a, P: AsRef<Path>>(p: P) -> anyhow::Result<TBoco<'a>> {
    let mut res = Vec::new();
    for r in std::fs::read_dir(p)?.filter_map(|s| s.ok()) {
        let p = r.path();
        let fname = ea_op(p.file_name(), "No filename")?;
        let fname = ea_op(fname.to_str(), "Not OS String")?;
        res.push(TData::String(String::from(fname)));
    }
    b_ok(TData::List(res))
}

pub fn is_file(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.to_string()), "No safe path")?;
            match std::fs::metadata(&sp) {
                Ok(md) => {
                    if !md.is_file() {
                        return b_ok(TData::Bool(false));
                    }
                }
                Err(_) => return b_ok(TData::Bool(false)),
            }
        }
        return b_ok(TData::Bool(true));
    })
}
pub fn is_dir(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.to_string()), "No safe path")?;
            match std::fs::metadata(&sp) {
                Ok(md) => {
                    if !md.is_dir() {
                        return b_ok(TData::Bool(false));
                    }
                }
                Err(_) => return b_ok(TData::Bool(false)),
            }
        }
        return b_ok(TData::Bool(true));
    })
}

pub fn dir(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        if l.len() == 1 {
            let sp = ea_op(safe_path(&pb, &l[0].to_string()), "Broken File path")?;
            return do_dir(sp);
        }
        let mut v = Vec::new();
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.to_string()), "Broken File path")?;
            v.push(do_dir(&sp)?.concrete());
        }
        b_ok(TData::List(v))
    })
}

pub fn file(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        let mut res = String::new();
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.to_string()), "Broken File path")?;
            let mut f = std::fs::File::open(sp)?;
            f.read_to_string(&mut res)?;
        }
        b_ok(TData::String(res))
    })
}

pub fn scan_dir(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |args| {
        let fm = BasicFuncs::new().with_defaults();
        let folder = args
            .get(0)
            .ok_or(ea_str("Folder Needed to scan"))?
            .to_string();
        let dp = args.get(1).and_then(|v| v.as_usize()).unwrap_or(0);
        let sp = ea_op(safe_path(&pb, &folder.to_string()), "Broken File path")?;
        let mut res = Vec::new();
        do_scan_dir(&sp, &PathBuf::new(), &fm, dp, &mut res)?;

        Ok(TBoco::Co(TData::List(res)))
    })
}

fn do_scan_dir(
    full: &Path,
    short: &Path,
    fm: &BasicFuncs,
    mdepth: usize,
    res_list: &mut Vec<TData>,
) -> anyhow::Result<()> {
    for item in std::fs::read_dir(full)?
        .filter_map(|s| s.ok())
        .filter(|v| !v.file_name().to_string_lossy().starts_with("_"))
    {
        let ftype = item.file_type()?;
        if ftype.is_file() {
            let mut idata = HashMap::new();
            idata.insert(
                "item_full_path".to_string(),
                TData::String(item.path().display().to_string()),
            );
            idata.insert(
                "item_path".to_string(),
                TData::String(short.display().to_string()),
            );
            idata.insert(
                "item_file".to_string(),
                TData::String(item.file_name().to_string_lossy().to_string()),
            );
            let mut s = String::new();
            let mut f = match std::fs::File::open(item.path()) {
                Ok(r) => r,
                Err(_) => continue,
            };
            f.read_to_string(&mut s).ok();
            if let Ok(tree) = TreeTemplate::from_str(&s) {
                let ex = tree.front_matter(fm);
                for (k, v) in ex {
                    idata.insert(k, v);
                }
            }
            res_list.push(TData::Map(idata));
        } else if ftype.is_dir() {
            if mdepth == 0 {
                continue;
            }
            let short = short.join(item.file_name());
            let full = full.join(item.file_name());
            do_scan_dir(&full, &short, fm, mdepth - 1, res_list)?;
        }
    }
    Ok(())
}
