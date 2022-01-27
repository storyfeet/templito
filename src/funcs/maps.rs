use crate::*;
use err_tools::*;
use std::collections::HashMap;
use std::ops::Deref;
use tparam::*;

pub fn map<'a>(args: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let mut it = args.iter();
    let mut res = HashMap::new();
    while let (Some(k), Some(v)) = (it.next(), it.next()) {
        match k.deref() {
            TData::String(s) => {
                res.insert(s.clone(), v.clone().into_owned());
            }
            _ => return e_str("The first part of each map-pair must be string"),
        }
    }
    Ok(TCow::Owned(TData::Map(res)))
}
