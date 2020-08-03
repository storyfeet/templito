use crate::*;
use pulldown_cmark as mdown;
pub fn cat(l: &[TData]) -> anyhow::Result<TData> {
    let mut r_str = String::new();
    for v in l {
        match v {
            TData::String(s) => r_str.push_str(s),
            _ => r_str.push_str(&v.to_string()),
        }
    }
    Ok(TData::String(r_str))
}

pub fn md(l: &[TData]) -> anyhow::Result<TData> {
    let mut r_str = String::new();
    for s in l {
        let pops = mdown::Options::all();
        let s = s.to_string();
        let p = mdown::Parser::new_ext(&s, pops);
        mdown::html::push_html(&mut r_str, p);
    }
    Ok(TData::String(r_str))
}
