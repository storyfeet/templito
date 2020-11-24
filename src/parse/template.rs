use super::*;
use crate::pipeline::Pipeline;
use crate::template::*;
use gobble::*;

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
        plus(last('.',Var)).map(|v|{
            let mut r = vec![VarPart::Num(0)];
            r.extend(v.into_iter());
            Pipeline::Var(r)
        }),
        ('@').map(|_| Pipeline::Var(vec![VarPart::Id("@".to_string())])),
        tdata::Data.map(|d| Pipeline::Lit(d)),
        (Ident,star(wn__(Pipe))).map(|(c,v)|Pipeline::Command(c,v)),
    )
}

parser! {(StringItem->String)
    or!(
        ("\\",WN.plus(),maybe("\\")).map(|_|String::new()),
        ("\\",Any.one()).map(|(_,c)|c.to_string()),
        ("{",fail_on("{")).map(|(s,_)|s.to_string()),
        WN.plus(),
        not(" \n\t\r{\\").plus(),
    )
}

parser! {(Assign->(String,Pipeline))
    (wn_(Ident),wn_("="),wn_(Pipe),ws_((or_ig!(";,\n".one(),peek(IClose)),WN.star()))).map(|(a,_,v,_)|(a,v))
}

parser! {(IOpen->())
    (or_ig!(wn_("{{-"),"{{"),WN.star()).ig()
}

parser! {(IClose->())
    wn_(or_ig!(("-}}",WN.star()),"}}"))
}

parser! {(Item->FlatItem)
    middle(IOpen,or!(
            Comment.map(|_|FlatItem::Comment),
            keyword("else").map(|_|FlatItem::Else),
            (keyword("if"),wn__(Pipe)).map(|(_,p)|FlatItem::If(p)),
            (keyword("return"),wn__(Pipe)).map(|(_,p)|FlatItem::Return(p)),
            (keyword("elif"),wn__(Pipe)).map(|(_,p)|FlatItem::Elif(p)),
            (keyword("for"),wn_(Ident),wn_(Ident),wn_(keyword("in")),wn__(Pipe)).map(|(_,k,v,_,p)| FlatItem::For(k,v,p)),
            (keyword("switch"),star(wn__(Pipe))).map(|(_,v)| FlatItem::Switch(v)),
            (keyword("case"),star(wn__(expr::Pat))).map(|(_,v)| FlatItem::Case(v)),
            (keyword("define"),wn__(Ident)).map(|(_,n)|FlatItem::Define(n)),
            (keyword("global"),wn__(Ident)).map(|(_,n)|FlatItem::Global(n)),
            (keyword("let"),plus(Assign)).map(|(_,v)|FlatItem::Let(v)),
            (keyword("export"),plus(Assign)).map(|(_,v)|FlatItem::Export(v)),
            (keyword("@let"),wn_(Ident)).map(|(_,n)|FlatItem::AtLet(n)),
            (keyword("@export"),wn_(Ident)).map(|(_,n)|FlatItem::AtExport(n)),
            ('@',Ident," \t\n".istar(),star(wn__(Pipe))).map(|(_,s,_,b)|FlatItem::Block(s,b)),
            ('/',Ident,WS.star()).map(|(_,n,_)|FlatItem::EndBlock(n)),
            Pipe.map(|p|FlatItem::Pipe(p)),
    ).brk(),wn_(IClose).brk())
        .or(strings_plus_until(StringItem,peek(or_ig!(IOpen,eoi))).map(|(s,_)|FlatItem::String(s)))
}
