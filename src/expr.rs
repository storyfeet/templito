use crate::boco::Boco;
use crate::func_man::FuncManager;
use crate::pipeline::Pipeline;
use crate::scope::Scope;
use crate::tdata::*;
use crate::temp_man::TempManager;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    List(Vec<Pattern>),
    Val(TData),
    Filter(Pipeline),
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
                let res = f.run(scope, tm, fm).map(Boco::concrete);
                scope.pop();
                match res {
                    Ok(TData::Bool(true)) => true,
                    _ => false,
                }
            }
        }
    }
}

pub enum Expr {
    Lit(TData),
    Pipe(Pipeline),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
}
