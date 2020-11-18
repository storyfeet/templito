use crate::*;
use boco::*;
use err_tools::*;
use pulldown_cmark as mdown;
use std::ops::Deref;
use tparam::*;

pub fn cat<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut r_str = String::new();
    for v in l {
        match v.deref() {
            TData::String(s) => r_str.push_str(s),
            r => r_str.push_str(&r.to_string()),
        }
    }
    Ok(TBoco::Co(TData::String(r_str)))
}

pub fn split<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return e_str("Nothing to split");
    }
    let splitter = l.get(1).and_then(|n| n.deref().as_str()).unwrap_or("\n");
    l[0].deref()
        .as_str()
        .e_str("To split Must be a string")
        .map(|v| {
            TBoco::Co(TData::List(
                v.split(splitter)
                    .map(|s| TData::String(s.to_string()))
                    .collect(),
            ))
        })
}

pub fn contains<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 2 {
        return e_str("'contains' requires a string and then a list of potential substrings");
    }
    let s = args[0].to_string();
    for sub in args[1..].iter().map(|v| v.to_string()) {
        if s.contains(&sub) {
            return b_ok(TData::Bool(true));
        }
    }
    b_ok(TData::Bool(false))
}

pub fn replace<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 3 {
        return e_str("'replace' requires a string then substr to replace with substr");
    }
    b_ok(TData::String(
        args[0]
            .to_string()
            .replace(&args[1].to_string(), &args[2].to_string()),
    ))
}
pub fn replace_n<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 3 {
        return e_str("'replace' requires a string then substr to replace with substr");
    }
    let n = args.get(3).and_then(|v| v.as_usize()).unwrap_or(1);
    b_ok(TData::String(args[0].to_string().replacen(
        &args[1].to_string(),
        &args[2].to_string(),
        n,
    )))
}

pub fn md<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut r_str = String::new();
    for s in l {
        let pops = mdown::Options::all();
        let s = s.deref().to_string();
        let p = mdown::Parser::new_ext(&s, pops);
        mdown::html::push_html(&mut r_str, p);
    }
    Ok(TBoco::Co(TData::String(r_str)))
}

pub fn html_esc<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut res = String::new();
    for a in args {
        for c in a.to_string().chars() {
            match c {
                '&' => res.push_str("&amp;"),
                '<' => res.push_str("&lt;"),
                '>' => res.push_str("&gt;"),
                '"' => res.push_str("&quot"),
                '\'' => res.push_str("&#39"),
                c => res.push(c),
            }
        }
    }
    Ok(TBoco::Co(TData::String(res)))
}

pub fn table<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() == 0 {
        return e_str("Table requires 1 or two string entries.");
    }
    let tdata = match l.get(1) {
        Some(v) => v.to_string(),
        None => String::new(),
    };
    super::table::table(&l[0].to_string(), &tdata).map(|s| TBoco::Co(TData::String(s)))
}

pub fn regex<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() < 2 {
        return e_str("Regex test requires String , Regex_String");
    }
    let reg = regex::Regex::new(&l[1].to_string())?;
    b_ok(TData::Bool(reg.is_match(&l[0].to_string())))
}

fn _word_wrap(s: &str, len: usize) -> Vec<String> {
    let mut res = Vec::new();
    let mut index = 0;
    let mut count = 0;
    let mut brk = None;
    for (k, c) in s.char_indices() {
        count += 1;
        match c {
            ' ' | '\t' => brk = Some(k),
            '\n' => {
                res.push(s[index..k].to_string());
                count = 0;
                index = k;
                brk = None;
            }
            _ => {}
        }
        if count >= len {
            if let Some(b) = brk {
                res.push(s[index..b].trim().to_string());
                index = b;
                count = 0;
                brk = None;
            }
        }
    }
    if count > 0 {
        res.push(s[index..].trim().to_string());
    }
    res
}

pub fn word_wrap<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if l.len() < 2 {
        return e_str("'word_wrap' requires string and wrap_length");
    }
    let n = l[1]
        .as_usize()
        .e_str("'word_wrap' needs a length as usize")?;
    let w = _word_wrap(&l[0].to_string(), n);
    return b_ok(TData::List(
        w.into_iter().map(|s| TData::String(s)).collect(),
    ));
}

pub fn debug<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    for a in l {
        eprintln!("{}", a);
    }
    b_ok(TData::String(String::new()))
}
