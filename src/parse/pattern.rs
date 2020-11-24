use super::*;
use crate::pattern::Pattern;
use expr::*;
use gobble::*;

parser! {(Pat ->Pattern)
    or!(
        keyword("_").asv(Pattern::Any),
        ("[",sep_until_ig(wn__(Pat),maybe(","),"]")).map(|(_,v)|Pattern::List(v)),
        ("?(",Exp,")").map(|(_,v,_)|Pattern::Filter(v)),
        ("<",wn__(Ident),maybe(":".ig_then(Pat)),">").map(|(_,i,op,_)|Pattern::Capture(i,op.map(|p|Box::new(p)))),

        SimpleData.map(|v|Pattern::Val(v)),
    )
}
