use crate::*;
use err::Error;
use func_man::FuncManager;
use scope::Scope;
use temp_man::TempManager;
use tparam::TParam;

#[derive(Clone, Debug, PartialEq)]
pub enum Pipeline {
    Lit(String),
    Var(Vec<VarPart>),
    Command(String, Vec<Pipeline>),
}

pub fn run_values<TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &[TData],
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TData> {
    if let Some(in_tp) = tm.get_t(cname).map(|t| t.clone()) {
        let v2: Vec<&dyn TParam> = Vec::new();
        for a in args {
            v2.push(a);
        }
        Ok(TData::String(in_tp.run(&v2, tm, fm)?))
    } else if let Some(in_f) = fm.get_func(&cname) {
        Ok(in_f(&args)?)
    } else {
        Err(Error::String(format!("No function or template b the name {}", cname)).into())
    }
}

pub fn run_command<TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &[Pipeline],
    scope: &Scope,
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TData> {
    if cname == "first_valid" {
        for p in args {
            if let Ok(res) = p.run(scope, tm, fm) {
                return Ok(res);
            }
        }
        return Err(Error::Str("No elements passed the existence test").into());
    }
    if cname == "select" {
        if args.len() < 3 {
            return Err(Error::Str("n_sel requires at least 1 test and 2 possible values").into());
        }
        let bval = args[0].run(scope, tm, fm)?;
        let bval = if let Some(n) = bval.as_usize() {
            n + 1
        } else if let Some(b) = bval.as_bool() {
            2 - (b as usize)
        } else {
            return Err(Error::String(format!("As Bool failed on {}", bval)).into());
        };

        if bval > args.len() {
            return Err(Error::Str("select first param must choose return a value less than the length of the rest of the args").into());
        }

        return args[bval].run(scope, tm, fm);
    }
    let mut v = Vec::new();
    for p in args {
        let pval = p.run(scope, tm, fm)?;
        //println!("Param value = {:?}", pval);
        v.push(pval);
    }
    run_values(cname, &v, tm, fm)
}

impl Pipeline {
    pub fn run<TM: TempManager, FM: FuncManager>(
        &self,
        scope: &Scope,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<TData> {
        println!("Running = {:?}", self);
        match self {
            Pipeline::Lit(v) => Ok(TData::from_str(&v)?),
            Pipeline::Var(v) => scope
                .get(v)
                .map(|v| v.clone())
                .ok_or(Error::String(format!("No Var by the name {:?}", v)).into()),
            Pipeline::Command(c, pars) => run_command(c, pars, scope, tm, fm),
        }
    }
}
