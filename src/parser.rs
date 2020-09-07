use crate::pipeline::Pipeline;
use crate::template::*;
use gobble::*;

char_bool!(WN, " \t\n\r");

pub fn wn_<P: Parser>(p: P) -> impl Parser<Out = P::Out> {
    last(WN.star(), p)
}
pub fn wn__<P: Parser>(p: P) -> impl Parser<Out = P::Out> {
    middle(WN.star(), p, WN.star())
}

parser! { (Ident->String)
    or! (common::Ident,
        middle('\'',not('\'').plus(),'\'')
    )
}

parser! { (TFile->FlatTemplate)
    star_until_ig(Item,eoi).map(|v|FlatTemplate{v})
}

parser! {(Var->VarPart)
    or!(Ident.map(|s|VarPart::Id(s)),
        common::UInt.map(|u|VarPart::Num(u)))
}

parser! {(Comment->())
    ('#',not('#').istar(),'#').ig()
}

parser! {(Pipe->Pipeline)
    or!(
        middle(wn_('('),wn__(Pipe),')'),
        ('$',sep_star(Var,'.')).map(|(_,p)|Pipeline::Var(p)),
        ('@').map(|_| Pipeline::Var(vec![VarPart::Id("@".to_string())])),
        crate::td_parser::Data.map(|d| Pipeline::Lit(d)),
        (Ident,star(wn__(Pipe))).map(|(c,v)|Pipeline::Command(c,v)),
    )
}

parser! {(StringItem->String)
    or!(
        ("\\",("\t \n\r").plus(),maybe("\\")).map(|_|String::new()),
        ("\\",Any.one()).map(|(_,c)|c.to_string()),
        string(("{",not("{\\").plus())),
        not("{\\").plus(),
    )
}

parser! {(Assign->(String,Pipeline))
    (wn_(Ident),wn_("="),wn_(Pipe),ws_((or_ig!(";,\n".one(),peek('}')),WN.star()))).map(|(a,_,v,_)|(a,v))
}

parser! {(Item->FlatItem)
    middle("{{",or!(
            wn__(Comment).map(|_|FlatItem::Comment),
            wn__(keyword("else")).map(|_|FlatItem::Else),
            wn__(keyword("/if")).map(|_|FlatItem::EndIf),
            wn__(keyword("/for")).map(|_|FlatItem::EndFor),
            (wn_(keyword("if")),wn__(Pipe)).map(|(_,p)|FlatItem::If(p)),
            (wn_(keyword("elif")),wn__(Pipe)).map(|(_,p)|FlatItem::Elif(p)),
            (wn_(keyword("for")),wn_(Ident),wn_(Ident),wn_(keyword("in")),wn__(Pipe)).map(|(_,k,v,_,p)| FlatItem::For(k,v,p)),
            (wn_(keyword("define")),wn__(Ident)).map(|(_,n)|FlatItem::Define(n)),
            (wn_(keyword("let")),plus(Assign)).map(|(_,v)|FlatItem::Let(v)),
            (wn_(keyword("export")),plus(Assign)).map(|(_,v)|FlatItem::Export(v)),
            (wn_(keyword("@let")),wn_(Ident)).map(|(_,n)|FlatItem::AtLet(n)),
            (wn_(keyword("@export")),wn_(Ident)).map(|(_,n)|FlatItem::AtExport(n)),
            (wn_('@'),Ident," \t\n".istar(),star(wn__(Pipe))).map(|(_,s,_,b)|FlatItem::Block(s,b)),
            (wn_('/'),Ident,WS.star()).map(|(_,n,_)|FlatItem::EndBlock(n)),
            wn__(Pipe).map(|p|FlatItem::Pipe(p)),
    ).brk(),wn_("}}").brk())
        .or(strings_plus_until(StringItem,peek(or_ig!("{{",eoi))).map(|(s,_)|FlatItem::String(s)))
}
