use super::*;
use crate::scope::Scope;
use crate::temp_man::NoTemplates;
use err_tools::*;
use expr::VarPart;
use gobble::*;
use parse::expr::*;
use std::cmp::Ordering;
use std::ops::Deref;
use tdata::*;
use tparam::*;

pub fn list<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let v = args.iter().map(|b| b.clone().into_owned()).collect();
    b_ok(TData::List(v))
}

pub fn len<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let mut res = 0;
    for a in args {
        match a.deref() {
            TData::List(l) => res += l.len(),
            TData::Map(m) => res += m.len(),
            _ => res += 1,
        }
    }
    b_ok(TData::UInt(res))
}

pub fn slice<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let start = args
        .get(1)
        .and_then(|v| v.deref().as_usize())
        .e_str("slice: no start point?")?;

    let end = args.get(2).and_then(|v| v.deref().as_usize());

    let s = match (args[0].deref(), end) {
        (TData::List(l), Some(e)) => l.get(start..e).e_str("slice out of range")?,
        (TData::List(l), None) => l.get(start..).e_str("slice out of range")?,

        (TData::String(s), Some(e)) => {
            return b_ok(TData::String(
                s.get(start..e).e_str("slice out of range")?.to_string(),
            ))
        }
        (TData::String(s), None) => {
            return b_ok(TData::String(
                s.get(start..).e_str("slice out of range")?.to_string(),
            ))
        }
        _ => return e_str("Slice requires List Num Num?"),
    };
    let v: Vec<TData> = s.iter().map(|d| (*d).clone()).collect();
    b_ok(TData::List(v))
}

pub fn index_of<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 2 {
        return e_str("index_of requires 'needle' then 'haystack'")
    }
    let needle = args[0].deref();
    if let TData::List(l) = args[1].deref(){
        for (k,v) in l.into_iter().enumerate(){
            if v.eq(needle){
                return b_ok(TData::UInt(k))
            }
        }
    }
    return b_ok(TData::Null)
}

pub fn sort<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
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

fn build_comparator<'a>(args: &[TCow<'a>]) -> anyhow::Result<Vec<Comparator>> {
    let mut comps = Vec::new();
    for x in args {
        if let TData::String(s) = x.deref() {
            comps.push(
                Compare
                    .parse_s(s)
                    .map_err(|e| e.strung())
                    .e_str("Not a comparator string")?,
            );
        } else {
            return e_str("Sort On must have strings for idents");
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

pub fn sort_on<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 2 {
        return e_str("Sort requires a List<Map> and then at least one string");
    }
    // build comparisons
    let comps = build_comparator(&args[1..])?;
    if let TData::List(l) = args[0].deref() {
        let mut l = l.clone();
        l.sort_by(|a, b| compare(&comps, a, b));
        b_ok(TData::List(l))
    } else {
        e_str("Sort requires a List<Map> and then at least one string")
    }
}

pub fn bin_search<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 3 {
        return e_str("Bin Search needs a List<Map> an Item, and at least one sort criteria");
    }
    let list = match args[0].deref() {
        TData::List(l) => l,
        _ => return e_str("First Item must be a list"),
    };
    let comps = build_comparator(&args[2..])?;
    Ok(list
        .binary_search_by(|v| compare(&comps, v, args[1].deref()))
        .map(|n| TCow::Owned(TData::UInt(n)))
        .unwrap_or(TCow::Owned(TData::Null)))
}

pub fn bin_get<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 3 {
        return e_str("Bin Get needs a List<Map> an Item, and at least one sort criteria");
    }
    let list = match args[0].deref() {
        TData::List(l) => l,
        _ => return e_str("First Item must be a list"),
    };
    let comps = build_comparator(&args[2..])?;
    let n = list.binary_search_by(|v| compare(&comps, v, args[1].deref()));
    match n {
        Ok(n) => Ok(args[n].clone()),
        Err(_) => Ok(TCow::Owned(TData::Null)),
    }
}

pub fn get<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let pos = match args[1].deref() {
        TData::Int(i) => TData::UInt(*i as usize),
        v => v.clone(),
    };
    match args.len() {
        0 | 1 => e_str("Get requires List and Number"),
        2 => match (args[0].deref(), &pos) {
            (TData::List(l), TData::UInt(u)) => Ok(l
                .get(*u)
                .map(|v| TCow::Owned(v.clone()))
                .unwrap_or(TCow::Owned(TData::Null))),
            (TData::String(s),TData::UInt(u))=>{
                
                let c = s.get(*u..).and_then(|s| s.chars().next()).e_str("No char at that point")?;
                let mut s2 = String::new();
                s2.push(c);
                b_ok(TData::String(s2))
            }
            (TData::Map(m), TData::String(s))=> Ok(m.get(s).map(|v| TCow::Owned(v.clone())).unwrap_or(TCow::Owned(TData::Null))),
            _=>e_str("get requires either List and Int or Map and String"),
        },
        _=>e_str("Get requires List and exactly 1 Position marker. Multiple markers may be allowed in the future")
    }
}

pub fn filter<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    if args.len() < 2 {
        return e_str("Filter requires List and PipeString");
    }
    let fm = BasicFuncs::new().with_defaults();
    let pp = Exp.parse_s(&args[1].to_string()).map_err(|e| e.strung())?;
    if let TData::List(ls) = args[0].deref() {
        return b_ok(TData::List(
            ls.iter()
                .filter_map(|v| {
                    let params: [&dyn TParam; 1] = [v];
                    let scope = Scope::new(&params[..]);
                    match pp.run(&scope, &mut NoTemplates, &fm) {
                        Ok(b) if b.as_bool().unwrap_or(false) => Some(v.clone()),
                        _ => None,
                    }
                })
                .collect(),
        ));
    } else {
        e_str("Filter requires List and PipeString")
    }
}

pub fn groups<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let n = args
        .get(0)
        .e_str("'groups' requires an int and a list")?
        .as_usize()
        .e_str("'groups' requires an int and a list")?;
    let gsize = match n {
        0 => 1,
        v => v,
    };
    let l = args.get(1).e_str("'groups' requires an int and a list")?;
    let l = match l.deref() {
        TData::List(l) => l,
        _ => return e_str("'groups' second arg must be a list"),
    };
    let mut res = Vec::new();
    if l.len() == 0 {
        return b_ok(TData::List(res));
    }
    for g in 0..1 + ((l.len() - 1) / gsize) {
        let end = ((g + 1) * gsize).min(l.len());
        res.push(TData::List((&l[g * gsize..end]).to_vec()));
    }
    assert_eq!(
        res.len(),
        ((l.len() - 1) / gsize) + 1,
        "GroupSize Not match"
    );
    b_ok(TData::List(res))
}

pub fn append<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let mut res = Vec::new();
    for a in args {
        match a.deref() {
            TData::List(l) => res.extend(l.into_iter().map(|d| d.clone())),
            v => res.push(v.clone()),
        }
    }
    return b_ok(TData::List(res));
}
