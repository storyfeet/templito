use crate::*;
use err::Error;
use gobble::Parser;
use parser::TFile;
use scope::Scope;

#[derive(Clone)]
pub struct Template {
    pub v: Vec<TItem>,
}

#[derive(Clone)]
pub enum TItem {
    String(String),
    Pipe(TPipeline),
    If(TPipeline),
    Else,
    Elif(TPipeline),
    For(String, String, TPipeline),
    Let(Vec<(String, TPipeline)>),
    EndIf,
    EndFor,
}

#[derive(Clone, Debug)]
pub enum VarPart {
    Num(usize),
    Id(String),
}
#[derive(Clone)]
pub enum TPipeline {
    Lit(String),
    Var(Vec<VarPart>),
    Command(String, Vec<TPipeline>),
}

impl Template {
    pub fn run<D: Templable, TM: TempManager, FM: FuncManager<D>>(
        &self,
        params: &[D],
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<String> {
        let mut res = String::new();
        let mut scope = Scope::new(params);
        let mut it = (&self.v).into_iter();
        while let Some(item) = it.next() {
            match item {
                TItem::String(s) => res.push_str(s),
                TItem::Let(vec) => {
                    for (k, v) in vec {
                        let vsolid = v.run(&scope, tm, fm)?;
                        scope.set(k.to_string(), vsolid);
                    }
                }
                _ => {}
            }
        }
        Ok(res)
    }
}

impl TPipeline {
    pub fn run<D: Templable, TM: TempManager, FM: FuncManager<D>>(
        &self,
        scope: &Scope<D>,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<D> {
        match self {
            TPipeline::Lit(v) => Ok(D::parse_lit(&v)?),
            TPipeline::Var(v) => scope
                .get(v)
                .map(|v| v.clone())
                .ok_or(Error::String(format!("No Var by the name {:?}", v)).into()),
            TPipeline::Command(c, pars) => {
                if c == "first_true" {
                    for p in pars {
                        if let Ok(res) = p.run(scope, tm, fm) {
                            if let Some(true) = res.as_bool() {
                                return Ok(res);
                            }
                        }
                    }
                    return Err(Error::Str("No elements passed the existence test").into());
                }
                let mut v = Vec::new();
                for p in pars {
                    v.push(p.run(scope, tm, fm)?);
                }
                if let Some(in_tp) = tm.get(&c).map(|t| t.clone()) {
                    Ok(D::parse_lit(&in_tp.run(&v, tm, fm)?)?)
                } else if let Some(in_f) = fm.get_func(&c) {
                    Ok(in_f(&v)?)
                } else {
                    Err(Error::String(format!("No function or template b the name {}", c)).into())
                }
            }
        }
    }
}
impl std::str::FromStr for Template {
    type Err = gobble::StrungError;
    fn from_str(s: &str) -> Result<Template, Self::Err> {
        TFile.parse_s(s).map_err(|e| e.strung(s.to_string()))
    }
}
