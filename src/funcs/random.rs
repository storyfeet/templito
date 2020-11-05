use super::*;
use crate::err::ea_str;
use boco::*;
use rand::prelude::*;
use std::ops::Deref;
use tdata::*;
use tparam::*;

pub fn get_rand<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    let mut rg = rand::thread_rng();
    if args.len() == 0 {
        return b_ok(TData::Float(rg.gen()));
    }

    if args.len() == 2 {
        return match (args[0].deref(), args[1].deref()) {
            (TData::Int(a), TData::Int(b)) => b_ok(TData::Int(rg.gen_range(a, b))),
            (TData::UInt(a), TData::UInt(b)) => b_ok(TData::UInt(rg.gen_range(a, b))),
            (TData::Float(a), TData::Float(b)) => b_ok(TData::Float(rg.gen_range(a, b))),
            _ => Err(ea_str("Could not gen random")),
        };
    }

    match args[0].deref() {
        TData::Int(n) => b_ok(TData::Int(rg.gen_range(0, n))),
        TData::UInt(n) => b_ok(TData::UInt(rg.gen_range(0, n))),
        TData::Float(n) => b_ok(TData::Float(rg.gen::<f64>() * n)),
        TData::List(l) => {
            if l.len() == 0 {
                Err(ea_str("Could not select random from empty list"))
            } else {
                let n = rg.gen_range(0, l.len());
                Ok(TBoco::Co(l[n].clone()))
            }
        }
        _ => Err(ea_str("Could not gen random")),
    }
}
