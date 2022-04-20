use super::*;
use crate::pattern::Pattern;
use expr::*;
use gobble::*;

parser! {(PatKV->(String,Pattern))
    (wn__(Ident),maybe((":",wn__(Pat)).map(|(_,p)|p))).map(|(id,pop)|{
        match pop{
            Some(p)=>(id,p),
            None=>(id.to_string(),Pattern::Capture(id,None)),
        }
    })
}

parser! {(Pat ->Pattern)
    or!(
        keyword("_").asv(Pattern::Any),
        ("{",sep_until_ig(wn__(PatKV),maybe(","),"}")).map(|(_,v)|Pattern::Map(v)),
        ("[",sep_until_ig(wn__(Pat),maybe(","),"]")).map(|(_,v)|Pattern::List(v)),
        ("?(",Exp,")").map(|(_,v,_)|Pattern::Filter(v)),
        ("<",wn__(Ident),maybe(":".ig_then(Pat)),">").map(|(_,i,op,_)|Pattern::Capture(i,op.map(|p|Box::new(p)))),

        SimpleData.map(|v|Pattern::Val(v)),
    )
}
