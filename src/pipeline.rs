use crate::*;
use err::Error;
use func_man::FuncManager;
use scope::Scope;
use temp_man::TempManager;

#[derive(Clone, Debug, PartialEq)]
pub enum Pipeline {
    Lit(String),
    Var(Vec<VarPart>),
    Command(String, Vec<Pipeline>),
}

pub fn run_command<D: Templable, TM: TempManager, FM: FuncManager<D>>(
    cname: &str,
    args: &[Pipeline],
    scope: &Scope<D>,
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<D> {
    if cname == "first_valid" {
        for p in args {
            if let Ok(res) = p.run(scope, tm, fm) {
                if res.is_valid() {
                    return Ok(res);
                }
            }
        }
        return Err(Error::Str("No elements passed the existence test").into());
    }
    if cname == "b_sel" {
        if args.len() < 3 {
            return Err(Error::Str("b_sel requires at least 1 test and 2 possible values").into());
        }
        let bval = args[0].run(scope, tm, fm)?;
        let bval = bval
            .as_bool()
            .ok_or(Error::String(format!("As Bool failed on {}", bval)))?;
        match bval {
            true => return args[1].run(scope, tm, fm),
            false => return args[2].run(scope, tm, fm),
        }
    }
    if cname == "n_sel" {
        if args.len() < 3 {
            return Err(Error::Str("n_sel requires at least 1 test and 2 possible values").into());
        }
        let bval = args[0].run(scope, tm, fm)?;
        let bval = bval
            .as_usize()
            .ok_or(Error::String(format!("As Bool failed on {}", bval)))?
            + 1;

        if bval > args.len() {
            return Err(Error::Str("n_sel first param must return a value less than the length of the rest of the args").into());
        }

        return args[bval].run(scope, tm, fm);
    }
    let mut v = Vec::new();
    for p in args {
        let pval = p.run(scope, tm, fm)?;
        println!("Param value = {:?}", pval);
        v.push(pval);
    }

    if v.len() > 0 {
        if let Some(in_item) = v[0].get_func(cname) {
            return Ok(in_item(&v[0], &v[1..])?);
        }
    }
    if let Some(in_tp) = tm.get_t(cname).map(|t| t.clone()) {
        Ok(D::parse_lit(&in_tp.run(&v, tm, fm)?)?)
    } else if let Some(in_f) = fm.get_func(&cname) {
        Ok(in_f(&v)?)
    } else {
        Err(Error::String(format!("No function or template b the name {}", cname)).into())
    }
}

impl Pipeline {
    pub fn run<D: Templable, TM: TempManager, FM: FuncManager<D>>(
        &self,
        scope: &Scope<D>,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<D> {
        println!("Running = {:?}", self);
        match self {
            Pipeline::Lit(v) => Ok(D::parse_lit(&v)?),
            Pipeline::Var(v) => scope
                .get(v)
                .map(|v| v.clone())
                .ok_or(Error::String(format!("No Var by the name {:?}", v)).into()),
            Pipeline::Command(c, pars) => run_command(c, pars, scope, tm, fm),
        }
    }
}
