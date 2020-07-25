use crate::*;
use err::Error;
use scope::Scope;

#[derive(Clone, Debug, PartialEq)]
pub enum Pipeline {
    Lit(String),
    Var(Vec<VarPart>),
    Command(String, Vec<Pipeline>),
}
impl Pipeline {
    pub fn run<D: Templable, TM: TempManager, FM: FuncManager<D>>(
        &self,
        scope: &Scope<D>,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<D> {
        match self {
            Pipeline::Lit(v) => Ok(D::parse_lit(&v)?),
            Pipeline::Var(v) => scope
                .get(v)
                .map(|v| v.clone())
                .ok_or(Error::String(format!("No Var by the name {:?}", v)).into()),
            Pipeline::Command(c, pars) => {
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
