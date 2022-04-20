use crate::expr::Expr;
use crate::func_man::FuncManager;
use crate::scope::Scope;
use crate::tdata::*;
use crate::temp_man::TempManager;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Map(Vec<(String, Pattern)>),
    List(Vec<Pattern>),
    Val(TData),
    Filter(Expr),
    Capture(String, Option<Box<Pattern>>),
    Any,
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
            Pattern::Map(mpat) => match d {
                TData::Map(mp) => {
                    for (k, p) in mpat {
                        match mp.get(k) {
                            Some(v) => {
                                if !p.match_data(v, scope, tm, fm) {
                                    return false;
                                }
                            }
                            None => return false,
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
                let res = f.run(scope, tm, fm).map(Cow::into_owned);
                scope.pop();
                match res {
                    Ok(TData::Bool(true)) => true,
                    _ => false,
                }
            }
        }
    }
}
