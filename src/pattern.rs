use crate::func_man::FuncManager;
use crate::parser::{wn__, Ident, Pipe};
use crate::pipeline::Pipeline;
use crate::scope::Scope;
use crate::td_parser::*;
use crate::tdata::*;
use crate::temp_man::TempManager;
use gobble::*;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    List(Vec<Pattern>),
    Val(TData),
    Filter(Pipeline),
    Capture(String, Option<Box<Pattern>>),
    Any,
}

parser! {(Pat ->Pattern)
    or!(
        keyword("_").asv(Pattern::Any),
        ("[",star(wn__(Pat)),"]").map(|(_,v,_)|Pattern::List(v)),
        ("?(",Pipe,")").map(|(_,v,_)|Pattern::Filter(v)),
        ("<",wn__(Ident),maybe(":".ig_then(Pat)),">").map(|(_,i,op,_)|Pattern::Capture(i,op.map(|p|Box::new(p)))),

        Data.map(|v|Pattern::Val(v)),
    )
}

impl Pattern {
    pub fn match_data<TM: TempManager, FM: FuncManager>(
        &self,
        d: &TData,
        scope: &mut Scope,
        tm: &mut TM,
        fm: &FM,
    ) -> bool {
        match self {
            Pattern::Any => true,
            Pattern::Val(v) => v == d,
            Pattern::List(pl) => match d {
                TData::List(dl) => {
                    for (n, v) in pl.iter().enumerate() {
                        if n >= dl.len() {
                            return false;
                        }
                        if !v.match_data(&dl[n], scope, tm, fm) {
                            return false;
                        }
                    }
                    true
                }
                _ => false,
            },
            Pattern::Capture(id, Some(test)) => {
                if test.match_data(d, scope, tm, fm) {
                    scope.set(id, d.clone());
                    true
                } else {
                    false
                }
            }
            Pattern::Capture(id, None) => {
                scope.set(id, d.clone());
                true
            }
            Pattern::Filter(f) => {
                scope.push();
                scope.set("@".to_string(), d.clone());
                let res = f.run(scope, tm, fm);
                scope.pop();
                match res{
                    //TODO
                }
            }
        }
    }
}
