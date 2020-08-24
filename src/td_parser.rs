use crate::parser::wn__;
use crate::TData;
use gobble::*;
use std::collections::HashMap;

parser! {(Data->TData)
    wn__(or!(
        "null".map(|_|TData::Null),
        common::Bool.map(|b|TData::Bool(b)),
        common::Float.map(|f|TData::Float(f)),
        common::UInt.map(|n|TData::UInt(n)),
        common::Int.map(|i|TData::Int(i)),
        TString.map(|s|TData::String(s)),
        "[".ig_then(sep_until_ig(Data, ",", "]"))
            .map(|a| TData::List(a)),
        "{".ig_then(sep_until_ig(wn__(MapItem), ",", "}")).map(|a| {
            let mut m = HashMap::new();
            for (k, v) in a {
                m.insert(k, v);
            }
            TData::Map(m)
        })

    ))
}

parser! { (HexUnicode ->char)
    HexDigit
        .exact(4)
        .try_map(|v| {
            let n: u32 =
                u32::from_str_radix(&v, 16).map_err(|_| Expected::Str("4 hex digits"))?;
            std::char::from_u32(n).ok_or(Expected::Str("4 Hex digits"))
        })
        .brk()

}

parser! {(Escape -> char )
    '\\'.ig_then(or!(
        'b'.asv('\u{08}'),
        'f'.asv('\u{0C}'),
        'n'.asv('\n'),
        'r'.asv('\r'),
        't'.asv('\t'),
        'u'.ig_then(HexUnicode),
        "\"\\/".one(),
    )),

}

parser!(
    (TChar -> char)
    or!(
        Escape,
        not("\\\"").one()
    )
);

parser! {
    (TString->String)
    "\"".ig_then(chars_until(TChar, '"')).map(|(a, _)| a)
}

parser! {
    (MapItem->(String,TData))
    (wn__(TString),":", Data).map(|(a, _, b)| (a, b))
}
