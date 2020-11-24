use super::*;
use crate::TData;
use gobble::*;
use std::collections::HashMap;

parser! {(SData->TData)
    wn__(Data)
}

parser! {(Month->usize)
    or!(
        common::UInt.try_map(|v| if v < 1 || v > 12 {
            Err(Expected::Str("int between 1 and 12"))
        }else{Ok(v)}),
        "jan".asv(1),
        "feb".asv(2),
        "mar".asv(3),
        "apr".asv(4),
        "may".asv(5),
        "jun".asv(6),
        "jul".asv(7),
        "aug".asv(8),
        "sep".asv(9),
        "oct".asv(10),
        "nov".asv(11),
        "dec".asv(12),
    )
}

parser! {(Date->chrono::NaiveDate)
    (common::UInt,"/",Month,"/",common::Int)
        .try_map(|(d,_,m,_,y)| chrono::NaiveDate::from_ymd_opt(y as i32,m as u32,d as u32)
            .ok_or(Expected::Str("Date Creation Failed")))

}

parser! {(Data->TData)
    or!(
        "null".map(|_|TData::Null),
        common::Bool.map(|b|TData::Bool(b)),
        Date.map(|d|TData::Date(d)),
        common::Float.map(|f|TData::Float(f)),
        common::UInt.map(|n|TData::UInt(n)),
        common::Int.map(|i|TData::Int(i)),
        TString.map(|s|TData::String(s)),
        "[".ig_then(sep_until_ig(SData, ",", "]"))
            .map(|a| TData::List(a)),
        "{".ig_then(sep_until_ig(wn__(MapItem), ",", "}")).map(|a| {
            let mut m = HashMap::new();
            for (k, v) in a {
                m.insert(k, v);
            }
            TData::Map(m)
        })

    )
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
    or!(
        "\"".ig_then(chars_until(TChar, '"')).map(|(a, _)| a),
        last("r#\"",chars_until(Any.one(),"\"#")).map(|(a,_)|a),
    )
}

parser! {
    (MapItem->(String,TData))
    (wn__(TString),":", SData).map(|(a, _, b)| (a, b))
}
