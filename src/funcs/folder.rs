use super::WithFuncs;
use crate::*;
use err_tools::*;
use func_man::*;
use image::GenericImageView;
use std::collections::HashMap;
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tparam::*;

pub fn safe_path(p: &Path, s: &str) -> Option<PathBuf> {
    let pres = p.join(s);
    if pres.starts_with(p) {
        return Some(pres);
    }
    None
}

fn do_dir<'a, P: AsRef<Path>>(p: P) -> anyhow::Result<TCow<'a>> {
    let mut res = Vec::new();
    for r in std::fs::read_dir(p)?.filter_map(|s| s.ok()) {
        let p = r.path();
        let fname = p.file_name().e_str("No filename")?;
        let fname = fname.to_str().e_str("Not OS String")?;
        res.push(TData::String(String::from(fname)));
    }
    b_ok(TData::List(res))
}

pub fn is_file(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        for d in l {
            let sp = safe_path(&pb, &d.to_string()).e_str("No safe path")?;
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
            let sp = safe_path(&pb, &d.to_string()).e_str("No safe path")?;
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
            let sp = safe_path(&pb, &l[0].to_string()).e_str("Broken File path")?;
            return do_dir(sp);
        }
        let mut v = Vec::new();
        for d in l {
            let sp = safe_path(&pb, &d.to_string()).e_str("Broken File path")?;
            v.push(do_dir(&sp)?.into_owned());
        }
        b_ok(TData::List(v))
    })
}

pub fn file(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        let mut res = String::new();
        for d in l {
            let sp = safe_path(&pb, &d.to_string()).e_str("Broken File path")?;
            let mut f = std::fs::File::open(sp)?;
            f.read_to_string(&mut res)?;
        }
        b_ok(TData::String(res))
    })
}

pub fn file_bytes(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        let mut res = Vec::new();
        for d in l {
            let sp = safe_path(&pb, &d.to_string()).e_str("Broken File path")?;
            let mut f = std::fs::File::open(sp)?;
            f.read_to_end(&mut res)?;
        }
        b_ok(TData::Bytes(res))
    })
}

pub fn file_img_dimensions(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |l| {
        if let TData::String(d) = l.get(0).e_str("file_img_dimensions needs path")?.deref() {
            let sp = safe_path(&pb, &d.to_string()).e_str("Broken File path")?;
            let img = image::open(sp)?;
            let mut map = HashMap::new();
            map.insert("w".to_string(), TData::UInt(img.width() as usize));
            map.insert("h".to_string(), TData::UInt(img.height() as usize));
            return b_ok(TData::Map(map));
        }
        e_str("file_img_dimensions path should be string")
    })
}

pub fn scan_dir(pb: PathBuf) -> Box<TFunc> {
    Box::new(move |args| {
        let fm = BasicFuncs::new().with_defaults();
        let folder = args.get(0).e_str("Folder Needed to scan")?.to_string();
        let dp = args.get(1).and_then(|v| v.as_usize()).unwrap_or(0);
        let sp = safe_path(&pb, &folder.to_string()).e_str("Broken File path")?;
        let mut res = Vec::new();
        let filter = args
            .get(2)
            .and_then(|a| regex::Regex::new(&a.to_string()).ok());

        do_scan_dir(&sp, &PathBuf::new(), &fm, dp, &mut res, filter.as_ref())?;

        Ok(TCow::Owned(TData::List(res)))
    })
}

fn do_scan_dir(
    full: &Path,
    short: &Path,
    fm: &BasicFuncs,
    mdepth: usize,
    res_list: &mut Vec<TData>,
    reg: Option<&regex::Regex>,
) -> anyhow::Result<()> {
    for item in std::fs::read_dir(full)?
        .filter_map(|s| s.ok())
        .filter(|v| !v.file_name().to_string_lossy().starts_with("_"))
    {
        let ftype = item.file_type()?;
        if ftype.is_file() {
            let mut idata = HashMap::new();
            let full_path = item.path().display().to_string();
            if let Some(rv) = reg.clone() {
                if !rv.is_match(&full_path) {
                    println!("Regex No Match '{}'", full_path);
                    continue;
                }
            }
            idata.insert("item_full_path".to_string(), TData::String(full_path));

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
            do_scan_dir(&full, &short, fm, mdepth - 1, res_list, reg)?;
        }
    }
    Ok(())
}
