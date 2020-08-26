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
        o_flip(
            match (a.get_v(&self.v), b.get_v(&self.v)) {
                (Some(ba), Some(bb)) => ba.partial_cmp(&bb).unwrap_or(Ordering::Equal),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            },
            self.down,
        )
    }
}

fn build_comparator<'a>(args: &[TBoco<'a>]) -> anyhow::Result<Vec<Comparator>> {
    let mut comps = Vec::new();
    for x in args {
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
    Ok(comps)
}

fn compare(comps: &[Comparator], a: &TData, b: &TData) -> Ordering {
    for c in comps {
        match c.compare(a, b) {
            Ordering::Equal => {}
            r => return r,
        }
    }
    Ordering::Equal
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
    let comps = build_comparator(&args[1..])?;
    if let TData::List(l) = args[0].deref() {
        let mut l = l.clone();
        l.sort_by(|a, b| compare(&comps, a, b));
        b_ok(TData::List(l))
    } else {
        Err(ea_str(
            "Sort requires a List<Map> and then at least one string",
        ))
    }
}

pub fn bin_search<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 3 {
        return Err(ea_str(
            "Bin Search needs a List<Map> an Item, and at least one sort criteria",
        ));
    }
    let list = match args[0].deref() {
        TData::List(l) => l,
        _ => return Err(ea_str("First Item must be a list")),
    };
    let comps = build_comparator(&args[2..])?;
    Ok(list
        .binary_search_by(|v| compare(&comps, v.deref(), args[1].deref()))
        .map(|n| TBoco::Co(TData::UInt(n)))
        .unwrap_or(TBoco::Co(TData::Null)))
}

pub fn bin_get<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() < 3 {
        return Err(ea_str(
            "Bin Get needs a List<Map> an Item, and at least one sort criteria",
        ));
    }
    let list = match args[0].deref() {
        TData::List(l) => l,
        _ => return Err(ea_str("First Item must be a list")),
    };
    let comps = build_comparator(&args[2..])?;
    let n = list.binary_search_by(|v| compare(&comps, v.deref(), args[1].deref()));
    match n {
        Ok(n) => Ok(args[n].clone()),
        Err(_) => Ok(TBoco::Co(TData::Null)),
    }
}

pub fn get<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    match args.len() {
        0 | 1 => Err(ea_str("Get requires List and Number")),
        2 => match (args[0].deref(), args[1].deref()) {
            (TData::List(l), TData::UInt(u)) => Ok(l
                .get(*u)
                .map(|v| TBoco::Co(v.clone()))
                .unwrap_or(TBoco::Co(TData::Null))),
            (TData::List(l), TData::Int(i)) => Ok(l
                .get(*i as usize)
                .map(|v| TBoco::Co(v.clone()))
                .unwrap_or(TBoco::Co(TData::Null))),
            (TData::Map(m), TData::String(s))=> Ok(m.get(s).map(|v| TBoco::Co(v.clone())).unwrap_or(TBoco::Co(TData::Null))),
            _=>Err(ea_str("get requires either List and Int or Map and String")),
        },
        _=>Err(ea_str("Get requires List and exactly 1 Position marker. Multiple markers may be allowed in the future"))
    }
}
