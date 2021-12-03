use super::*;
use crate::expr::*;
use crate::TData;
use gobble::*;

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

parser! {(Var->VarPart)
    or!(Ident.map(|s|VarPart::Id(s)),
        common::UInt.map(|u|VarPart::Num(u)))
}

parser! { (SimpleData->TData)
    or!(
        "null".map(|_|TData::Null),
        common::Bool.map(|b|TData::Bool(b)),
        Date.map(|d|TData::Date(d)),
        common::Float.map(|f|TData::Float(f)),
        common::UInt.map(|n|TData::UInt(n)),
        common::Int.map(|i|TData::Int(i)),
        TString.map(|s|TData::String(s)),
    )
}
parser! {
    (MapData->(String,TData))
    (wn__(TString),":", BasicData).map(|(a, _, b)| (a, b))
}

parser! {(BasicData->TData)
    or!(
        SimpleData,
        ("[",sep_until_ig(wn__(BasicData),maybe(","),"]")).map(|(_,v)|TData::List(v)),
        "{".ig_then(sep_until_ig(wn__(MapData), ",", "}")).map(|a|{
            let mut r = std::collections::HashMap::new();
            for (k,v) in a{
                r.insert(k,v);
            }
            TData::Map(r)
        }),
    )
}

parser! {(Exp->Expr)
    or!(
        middle('(',wn__(Exp),')'),
        ("[",sep_until_ig(wn__(Exp),maybe(","),"]")).map(|(_,v)|Expr::List(v)),
        "{".ig_then(sep_until_ig(wn__(MapItem), ",", "}")).map(|a| Expr::Map(a)),
        ('$',sep_star(Var,'.')).map(|(_,p)|Expr::Var(p)),
        plus(last('.',Var)).map(|v|{
            let mut r = vec![VarPart::Num(0)];
            r.extend(v.into_iter());
            Expr::Var(r)
        }),
        ('@').map(|_| Expr::Var(vec![VarPart::Id("@".to_string())])),
        SimpleData.map(|d| Expr::Lit(d)),
        (Ident,star(wn__(Exp))).map(|(c,v)|Expr::Command(c,v)),
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
    (MapItem->(String,Expr))
    (wn__(or!(Ident,TString)),":", Exp).map(|(a, _, b)| (a, b))
}
