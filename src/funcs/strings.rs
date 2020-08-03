use crate::*;
use pulldown_cmark as mdown;
pub fn cat(l: &[TData]) -> TData {
    let mut r_str = String::new();
    for v in l {
        match &v.as_str() {
            Some(s) => r_str.push_str(s),
            None => r_str.push_str(&v.to_string()),
        }
    }
    Ok(TData::string(&r_str))
}

pub fn md(l: &[TData]) -> anyhow::Result<TData> {
    let mut r_str = String::new();
    for s in l {
        let pops = mdown::Options::all();
        let s = s.string_it();
        let p = mdown::Parser::new_ext(&s, pops);
        mdown::html::push_html(&mut r_str, p);
    }
    Ok(TData::string("fooE"))
}
