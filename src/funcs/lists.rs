use crate::err::ea_str;
use crate::*;
use boco::*;
use gobble::*;
use parser::*;
use std::ops::Deref;
use template::VarPart;
use tparam::*;

pub fn list<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let v = args.iter().map(|b| b.clone().concrete()).collect();
    b_ok(TData::List(v))
}

pub fn sort<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut res = Vec::new();
    for a in args {
        if let TData::List(l) = a.deref() {
            for i in l {
                res.push(i.clone());
            }
        }
    }
    res.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    b_ok(TData::List(res))
}

pub struct Comparator {
    down: bool,
    v: Vec<VarPart>,
}

pub fn o_flip(o: Ordering, b: bool) -> Ordering {
    use Ordering::*;
    match (o, b) {
        (Greater, false) | (Less, true) => Greater,
        (Equal, _) => Equal,
        (Greater, true) | (Less, false) => Less,
    }
}

impl Comparator {
    fn compare(&self, a: &TData, b: &TData) -> Ordering {
        match (a.get_v(&self.v), b.get_v(&self.v)) {
            (Some(ba), Some(bb)) => {
                o_flip(ba.partial_cmp(&bb).unwrap_or(Ordering::Equal), self.down)
            }
            _ => o_flip(a.partial_cmp(b).unwrap_or(Ordering::Equal), self.down),
        }
    }
}

parser! {(Compare->Comparator)
    (exists("-"),sep_plus(Var,".")).map(|(down,v)| Comparator{down,v})
}

pub fn sort_on<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 2 {
        return Err(ea_str(
            "Sort requires a List<Map> and then at least one string",
        ));
    }
    // build comparisons
    let mut comps = Vec::new();
    for x in &args[1..] {
        if let TData::String(s) = x.deref() {
            comps.push(
                Compare
                    .parse_s(s)
                    .map_err(|_| ea_str("Not a comparator string"))?,
            );
        } else {
            return Err(ea_str("Sort On must have strings for idents"));
        }
    }

    if let TData::List(l) = args[0].deref() {
        let mut l = l.clone();
        l.sort_by(|a, b| {
            for c in &comps {
                match c.compare(a, b) {
                    Ordering::Equal => {}
                    r => return r,
                }
            }
            Ordering::Equal
        });
        b_ok(TData::List(l))
    } else {
        Err(ea_str(
            "Sort requires a List<Map> and then at least one string",
        ))
    }
}
