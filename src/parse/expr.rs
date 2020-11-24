use super::*;
use crate::expr::*;
use gobble::*;
use tdata::*;
use template::*;

parser! {(Pat ->Pattern)
    or!(
        keyword("_").asv(Pattern::Any),
        ("[",sep_until_ig(wn__(Pat),maybe(","),"]")).map(|(_,v)|Pattern::List(v)),
        ("?(",Pipe,")").map(|(_,v,_)|Pattern::Filter(v)),
        ("<",wn__(Ident),maybe(":".ig_then(Pat)),">").map(|(_,i,op,_)|Pattern::Capture(i,op.map(|p|Box::new(p)))),

        Data.map(|v|Pattern::Val(v)),
    )
}

parser! {(Exp->Expr)
    or!(
        ("[",sep_until_ig(wn__(Exp),maybe(","),"]")).map(|(_,v)|Expr::List(v)),
        Pipe.map(|p| Expr::Pipe(p)),
    )
}
