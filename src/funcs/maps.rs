use crate::err::ea_str;
use crate::*;
use std::collections::HashMap;
use std::ops::Deref;
use tparam::*;

pub fn map<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut it = args.iter();
    let mut res = HashMap::new();
    while let (Some(k), Some(v)) = (it.next(), it.next()) {
        match k.deref() {
            TData::String(s) => {
                res.insert(s.clone(), v.clone().concrete());
            }
            _ => return Err(ea_str("The first part of each map-pair must be string")),
        }
    }
    Ok(TBoco::Co(TData::Map(res)))
}
