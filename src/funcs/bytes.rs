use crate::*;
use err_tools::*;
use std::ops::Deref;
use tparam::*;

pub fn as_base64<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let b_ar = match l.get(0).e_str("needs 1 arg")?.deref() {
        TData::String(s) => s.as_bytes(),
        TData::Bytes(b) => b,
        _ => return e_str("as_base64 requires 1 string or bytes"),
    };
    let s = base64::encode(b_ar);
    b_ok(TData::String(s))
}

pub fn from_base64<'a>(l: &[TCow<'a>]) -> anyhow::Result<TCow<'a>> {
    let b_ar = match l.get(0).e_str("needs 1 arg")?.deref() {
        TData::String(s) => s.as_bytes(),
        TData::Bytes(b) => b,
        _ => return e_str("from_base64 requires 1 string or bytes"),
    };
    let s = base64::decode(b_ar)?;
    b_ok(TData::Bytes(s))
}
