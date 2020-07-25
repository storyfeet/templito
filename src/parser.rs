use crate::template::*;
use common::{Ident, Quoted};
use gobble::*;

parser! { (TFile->Template)
    star_until_ig(Item,eoi).map(|v|Template{v})
}

parser! {(Var->VarPart)
    or!(Ident.map(|s|VarPart::Id(s)),
        common::UInt.map(|u|VarPart::Num(u)))
}

parser! {(Pipeline->TPipeline)
    or!(
        middle('(',ws__(Pipeline),')'),
        ('$',sep_star(Var,'.')).map(|(_,p)|TPipeline::Var(p)),
        (Ident,star(ws__(Pipeline))).map(|(c,v)|TPipeline::Command(c,v)),
        string(Quoted).map(|v|TPipeline::Lit(v)),
        not(" \t}").plus().map(|v|TPipeline::Lit(v)),
    )
}

parser! {(StringChar->char)
    or("\\{".asv('{'),
    Any.one())
}

parser! {(Assign->(String,TPipeline))
    (ws_(Ident),ws_("="),ws_(Pipeline)).map(|(a,_,v)|(a,v))
}

parser! {(Item->TItem)
    middle("{{",or!(
            ws__(keyword("else")).map(|_|TItem::Else),
            ws__(keyword("/if")).map(|_|TItem::EndIf),
            ws__(keyword("/for")).map(|_|TItem::EndFor),
            (ws_(keyword("if")),ws__(Pipeline)).map(|(_,p)|TItem::If(p)),
            (ws_(keyword("elif")),ws__(Pipeline)).map(|(_,p)|TItem::Elif(p)),
            (ws_(keyword("for")),ws_(Ident),ws_(Ident),ws_(keyword("in")),ws__(Pipeline)).map(|(_,k,v,_,p)| TItem::For(k,v,p)),
            (ws_(keyword("let")),sep_plus(Assign,ws_(";"))).map(|(_,v)|TItem::Let(v)),
            Pipeline.map(|p|TItem::Pipe(p)),
    ),"}}")
        .or(chars_until(StringChar,peek(or_ig!("{{",eoi))).map(|(s,_)|TItem::String(s)))
}
