use crate::*;
//use card_format::Card;
use err_tools::*;
use gobble::traits::*;
use parse::expr::BasicData;
//use std::collections::HashMap;
use std::ops::Deref;
use tparam::*;
pub fn r_json<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let s = l.iter().next().e_str("r_json requires a string argument")?;
    if let TData::String(s) = s.deref() {
        return BasicData
            .parse_s(s)
            .map(|v| TCow::Owned(v))
            .map_err(|e| e.strung().into());
    }
    e_str("r_json requires a single string argument")
}

pub fn w_json<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    match l.len() {
        0 => b_ok(TData::String("[]".to_string())),
        1 => serde_json::to_string(&l[0])
            .map(|s| TCow::Owned(TData::String(s)))
            .map_err(|e| e.into()),
        _ => serde_json::to_string(l)
            .map(|s| TCow::Owned(TData::String(s)))
            .map_err(|e| e.into()),
    }
}

/*fn card_to_tdata(cd: &Card) -> TData {
    let mut res: HashMap<String, TData> = cd
        .data
        .iter()
        .map(|(k, v)| (k.to_string(), cdata_to_tdata(v)))
        .collect();
    res.insert("Name".to_string(), TData::String(cd.name.clone()));
    res.insert("Num".to_string(), TData::UInt(cd.num));
    TData::Map(res)
}

fn cdata_to_tdata(cd: &CData) -> TData {
    match cd {
        CData::L(l) => TData::List(l.into_iter().map(cdata_to_tdata).collect()),
        CData::R(v) | CData::S(v) => TData::String(v.clone()),
        CData::N(i) => TData::Int(*i),
    }
}*/

pub fn r_card<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let s = l.iter().next().e_str("r_json requires a string argument")?;
    if let TData::String(s) = s.deref() {
        return card_format::parse_cards(s)
            .map(|cs| TCow::Owned(TData::List(cs.iter().map(|c| c.into()).collect())))
            .map_err(|e| e.into());
    }
    e_str("r_json requires a single string argument")
}
