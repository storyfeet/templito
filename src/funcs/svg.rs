use super::*;
use crate::err::*;
use boco::*;
use tdata::*;
use tparam::*;

fn units(args: &[TBoco], p: usize) -> String {
    match args.get(p) {
        Some(v) => v.to_string(),
        _ => "px".to_string(),
    }
}

pub fn xy<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let u = units(args, 2);
    let y = args.get(1).e_str("need y")?;
    let x = args.get(0).e_str("need x")?;
    b_ok(TData::String(format!(r#"x="{}{2}" y="{}{2}""#, x, y, u)))
}

pub fn xywh<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let u = units(args, 4);
    let h = args.get(3).e_str("need h")?;
    let w = args.get(2).e_str("need w")?;
    let y = args.get(1).e_str("need y")?;
    let x = args.get(0).e_str("need x")?;
    b_ok(TData::String(format!(
        r#"x="{}{4}" y="{}{4}" w="{}{4}" h={}{4}"#,
        x, y, w, h, u
    )))
}

pub fn xy12<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let u = units(args, 4);
    let y2 = args.get(3).e_str("need y2")?;
    let x2 = args.get(2).e_str("need x2")?;
    let y = args.get(1).e_str("need y")?;
    let x = args.get(0).e_str("need x")?;
    b_ok(TData::String(format!(
        r#"x="{}{4}" y="{}{4}" x2="{}{4}" y2={}{4}"#,
        x, y, x2, y2, u
    )))
}

pub fn fl_stk<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let u = units(args, 3);
    let sw = args.get(2).e_str("need stroke-width")?;
    let s = args.get(1).e_str("need stroke")?;
    let f = args.get(0).e_str("need fill")?;
    b_ok(TData::String(format!(
        r#"fill="{}" stroke="{}" stroke-width="{}{3}""#,
        f, s, sw, u
    )))
}

pub fn font<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let sz = args.get(0).e_str("Font Size not supplied")?;
    let ff = args
        .get(1)
        .map(|s| format!(r#"font-family="{}" "#, s))
        .unwrap_or(String::new());

    let u = units(args, 2);
    b_ok(TData::String(format!(r#"font-size="{}{2}" {}"#, sz, ff, u)))
}

fn _xml_es(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        match c {
            '&' => res.push_str("&amp;"),
            '>' => res.push_str("&gt;"),
            '<' => res.push_str("&lt;"),
            '\"' => res.push_str("&quot;"),
            '\'' => res.push_str("&apos;"),
            cv => res.push(cv),
        }
    }
    res
}

pub fn xml_esc<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut res = String::new();
    for a in args {
        res.push_str(&_xml_es(&a.to_string()));
    }
    b_ok(TData::String(res))
}
