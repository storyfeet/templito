use crate::pipeline::Pipeline;
use crate::template::*;
use common::{Ident, Quoted};
use gobble::*;

parser! { (TFile->FlatTemplate)
    star_until_ig(Item,eoi).map(|v|FlatTemplate{v})
}

parser! {(Var->VarPart)
    or!(Ident.map(|s|VarPart::Id(s)),
        common::UInt.map(|u|VarPart::Num(u)))
}

parser! {(SingleQuotes->String)
    ('\'',chars_until(or!(not("\'\\").one(),"\\'".asv('\''),"\\\\".asv('\\')),'\''))
        .map(|(_,(v,_))|v)
}

parser! {(Pipe->Pipeline)
    or!(
        middle(ws_('('),ws__(Pipe),')'),
        ('$',sep_star(Var,'.')).map(|(_,p)|Pipeline::Var(p)),
        ('@').map(|_| Pipeline::Var(vec![VarPart::Id("@".to_string())])),
        (Ident,star(ws__(Pipe))).map(|(c,v)|Pipeline::Command(c,v)),
        string(Quoted).map(|v|Pipeline::Lit(v)),
        SingleQuotes.map(|v|Pipeline::Lit(v)),
        not(" \t}()").plus().map(|v|Pipeline::Lit(v)),
    )
}

parser! {(StringItem->String)
    or!(
        ("\\",("\t \n\r").plus(),maybe("\\")).map(|_|String::new()),
        ("\\",Any.one()).map(|(_,c)|c.to_string()),
        not("{\\").plus(),
    )
}

parser! {(Assign->(String,Pipeline))
    (ws_(Ident),ws_("="),ws_(Pipe)).map(|(a,_,v)|(a,v))
}

parser! {(Item->FlatItem)
    middle("{{",or!(
            ws__(keyword("else")).map(|_|FlatItem::Else),
            ws__(keyword("/if")).map(|_|FlatItem::EndIf),
            ws__(keyword("/for")).map(|_|FlatItem::EndFor),
            (ws_(keyword("if")),ws__(Pipe)).map(|(_,p)|FlatItem::If(p)),
            (ws_(keyword("elif")),ws__(Pipe)).map(|(_,p)|FlatItem::Elif(p)),
            (ws_(keyword("for")),ws_(Ident),ws_(Ident),ws_(keyword("in")),ws__(Pipe)).map(|(_,k,v,_,p)| FlatItem::For(k,v,p)),
            (ws_(keyword("let")),sep_plus(Assign,ws_(";"))).map(|(_,v)|FlatItem::Let(v)),
            (ws_('@'),Ident,star(ws__(Pipe))).map(|(_,s,b)|FlatItem::Block(s,b)),
            (ws_('/'),Ident,WS.star()).map(|(_,n,_)|FlatItem::EndBlock(n)),
            ws__(Pipe).map(|p|FlatItem::Pipe(p)),
    ),"}}")
        .or(strings_plus_until(StringItem,peek(or_ig!("{{",eoi))).map(|(s,_)|FlatItem::String(s)))
}
