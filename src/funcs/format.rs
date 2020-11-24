use crate::*;
use card_format::{CData, Card};
use err_tools::*;
use gobble::traits::*;
use parse::tdata::SData;
use std::collections::HashMap;
use std::ops::Deref;
use tparam::*;
pub fn r_json<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let s = l.iter().next().e_str("r_json requires a string argument")?;
    if let TData::String(s) = s.deref() {
        return SData
            .parse_s(s)
            .map(|v| TBoco::Co(v))
            .map_err(|e| e.strung().into());
    }
    e_str("r_json requires a single string argument")
}

fn card_to_tdata(cd: &Card) -> TData {
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
}

pub fn r_card<'a>(l: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let s = l.iter().next().e_str("r_json requires a string argument")?;
    if let TData::String(s) = s.deref() {
        return card_format::parse_cards(s)
            .map(|cs| TBoco::Co(TData::List(cs.iter().map(card_to_tdata).collect())))
            .map_err(|e| e.into());
    }
    e_str("r_json requires a single string argument")
}
