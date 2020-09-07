use crate::*;
//use anyhow::Context;
use boco::*;
use err::Error;
use func_man::FuncManager;
use scope::Scope;
use std::ops::Deref;
use temp_man::TempManager;
use tparam::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pipeline {
    Lit(TData),
    Var(Vec<VarPart>),
    Command(String, Vec<Pipeline>),
}

pub fn run_values<'a, TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &[TBoco<'a>],
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TBoco<'a>> {
    if let Some(in_f) = fm.get_func(&cname) {
        return Ok(in_f(&args)?);
    }
    match tm.get_t(cname).map(|t| t.clone()) {
        Ok(in_tp) => {
            let mut v2: Vec<&dyn TParam> = Vec::new();
            for a in args {
                v2.push(a);
            }
            b_ok(TData::String(in_tp.run(&v2, tm, fm)?))
        }
        Err(e) => Err(e.context(format!("Getting template {}", cname))),
    }
}

pub fn run_command<'a, TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &'a [Pipeline],
    scope: &'a Scope,
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TBoco<'a>> {
    if cname == "run" {
        println!("Running run");
        let mut tds = Vec::new();
        for p in args {
            tds.push(p.run(scope, tm, fm)?);
        }
        //Convert TParams to pointers to call the Template
        let mut v: Vec<&dyn TParam> = Vec::new();
        for p in &tds[1..] {
            v.push(p);
        }
        if let Some(tp) = tds.get(0) {
            if let TData::Template(t) = tp.deref() {
                return b_ok(TData::String(t.run(&v, tm, fm)?));
            }
        }
    }
    if cname == "first" {
        for p in args {
            if let Ok(res) = p.run(scope, tm, fm) {
                if res.deref() != &TData::Null {
                    return Ok(res);
                }
            }
        }
        return b_ok(TData::Null);
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
            return Err(Error::String(format!("As Bool failed on {}", bval.deref())).into());
        };

        if bval > args.len() {
            return Err(Error::Str("select first param must choose return a value less than the length of the rest of the args").into());
        }

        return args[bval].run(scope, tm, fm);
    }
    let mut v = Vec::new();
    for p in args {
        let pval = p.run(scope, tm, fm)?;
        v.push(pval);
    }
    run_values(cname, &v, tm, fm)
}

impl Pipeline {
    pub fn run<'a, TM: TempManager, FM: FuncManager>(
        &'a self,
        scope: &'a Scope,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<TBoco<'a>> {
        match self {
            Pipeline::Lit(v) => Ok(TBoco::Bo(v)),
            Pipeline::Var(v) => scope
                .get(v)
                .ok_or(Error::String(format!("No Var by the name {:?}", v)).into()),
            Pipeline::Command(c, pars) => run_command(c, pars, scope, tm, fm),
        }
    }
}
