use crate::*;
use boco::*;
use err::*;
use func_man::*;
use std::io::Read;
use std::path::{Path, PathBuf};
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

pub fn join() -> Box<TFunc> {
    Box::new(|l| {
        let mut res = PathBuf::new();
        for r in l {
            res.push(r.to_string())
        }
        b_ok(TData::String(res.display().to_string()))
    })
}
