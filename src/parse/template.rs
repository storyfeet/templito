use super::expr::Exp;
use super::*;
use crate::expr::Expr;
use crate::template::*;
use gobble::*;

parser! { (TFile->FlatTemplate)
    star_until_ig(Item,eoi).map(|v|FlatTemplate{v})
}

parser! {(Comment->())
    ('#',not('#').istar(),'#').ig()
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

parser! {(Assign->(String,Expr))
    (wn_(Ident),wn_("="),wn_(Exp),ws_((or_ig!(";,\n".one(),peek(IClose)),WN.star()))).map(|(a,_,v,_)|(a,v))
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
            (keyword("if"),wn__(Exp)).map(|(_,p)|FlatItem::If(p)),
            (keyword("return"),wn__(Exp)).map(|(_,p)|FlatItem::Return(p)),
            (keyword("elif"),wn__(Exp)).map(|(_,p)|FlatItem::Elif(p)),
            (keyword("for"),wn_(Ident),wn_(Ident),wn_(keyword("in")),wn__(Exp)).map(|(_,k,v,_,p)| FlatItem::For(k,v,p)),
            (keyword("switch"),star(wn__(Exp))).map(|(_,v)| FlatItem::Switch(v)),
            (keyword("case"),star(wn__(pattern::Pat))).map(|(_,v)| FlatItem::Case(v)),
            (keyword("as"),wn__(Exp),":",wn__(pattern::Pat)).map(|(_,v,_,p)|FlatItem::As(v,p)),
            (keyword("define"),wn__(Ident),star(wn__(Ident))).map(|(_,n,l)|FlatItem::Define(n,l)),
            (keyword("global"),wn__(Ident),star(wn__(Ident))).map(|(_,n,l)|FlatItem::Global(n,l)),
            (keyword("let"),plus(Assign)).map(|(_,v)|FlatItem::Let(v)),
            (keyword("export"),plus(Assign)).map(|(_,v)|FlatItem::Export(v)),
            (keyword("@let"),wn_(Ident)).map(|(_,n)|FlatItem::AtLet(n)),
            (keyword("@export"),wn_(Ident)).map(|(_,n)|FlatItem::AtExport(n)),
            ('@',Ident," \t\n".istar(),star(wn__(Exp))).map(|(_,s,_,b)|FlatItem::Block(s,b)),
            ('/',Ident,WS.star()).map(|(_,n,_)|FlatItem::EndBlock(n)),
            Exp.map(|p|FlatItem::Exp(p)),
    ).brk(),wn_(IClose).brk())
        .or(strings_plus_until(StringItem,peek(or_ig!(IOpen,eoi))).map(|(s,_)|FlatItem::String(s)))
}
