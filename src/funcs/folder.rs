use crate::*;
use err::*;
use func_man::*;
use std::io::Read;
use std::path::{Path, PathBuf};

fn safe_path(p: &Path, s: &str) -> Option<PathBuf> {
    let pres = p.join(s);
    if pres.starts_with(p) {
        return Some(pres);
    }
    None
}

fn do_dir<P: AsRef<Path>, D: Templable>(p: P) -> anyhow::Result<D> {
    let mut res = Vec::new();
    for r in std::fs::read_dir(p)?.filter_map(|s| s.ok()) {
        let p = r.path();
        let fname = ea_op(p.file_name(), "No filename")?;
        let fname = ea_op(fname.to_str(), "Not OS String")?;
        res.push(D::string(fname));
    }
    return ea_op(D::list(res), "Could not make list");
}

pub fn is_file<D: Templable>(pb: PathBuf) -> Box<TFunc<D>> {
    Box::new(move |l| {
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.string_it()), "No safe path")?;
            match std::fs::metadata(&sp) {
                Ok(md) => {
                    if !md.is_file() {
                        return Ok(D::bool(false));
                    }
                }
                Err(_) => return Ok(D::bool(false)),
            }
        }
        return Ok(D::bool(true));
    })
}
pub fn is_dir<D: Templable>(pb: PathBuf) -> Box<TFunc<D>> {
    Box::new(move |l| {
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.string_it()), "No safe path")?;
            match std::fs::metadata(&sp) {
                Ok(md) => {
                    if !md.is_dir() {
                        return Ok(D::bool(false));
                    }
                }
                Err(_) => return Ok(D::bool(false)),
            }
        }
        return Ok(D::bool(true));
    })
}

pub fn dir<D: Templable>(pb: PathBuf) -> Box<TFunc<D>> {
    Box::new(move |l| {
        if l.len() == 1 {
            let sp = ea_op(safe_path(&pb, &l[0].string_it()), "Broken File path")?;
            return do_dir(sp);
        }
        let mut v = Vec::new();
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.string_it()), "Broken File path")?;
            v.push(do_dir(&sp)?);
        }
        Ok(ea_op(D::list(v), "Cannot make list")?)
    })
}

pub fn file<D: Templable>(pb: PathBuf) -> Box<TFunc<D>> {
    Box::new(move |l| {
        let mut res = String::new();
        for d in l {
            let sp = ea_op(safe_path(&pb, &d.string_it()), "Broken File path")?;
            let mut f = std::fs::File::open(sp)?;
            f.read_to_string(&mut res)?;
        }
        Ok(D::string(&res))
    })
}

pub fn join<D: Templable>() -> Box<TFunc<D>> {
    Box::new(|l| {
        let res = PathBuf::new();
        for r in l {
            res.push(r.string_it())
        }
        Ok(D::string(&res.display().to_string()))
    })
}
