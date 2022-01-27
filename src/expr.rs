use crate::func_man::FuncManager;
use crate::scope::Scope;
use crate::tdata::*;
use crate::temp_man::TempManager;
use crate::tparam::*;
use err_tools::*;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub enum VarPart {
    Num(usize),
    Id(String),
}
impl VarPart {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            VarPart::Num(_) => None,
            VarPart::Id(s) => Some(s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(TData),
    Var(Vec<VarPart>),
    Command(String, Vec<Expr>),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
}

pub fn run_values<'a, TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &[TCow<'a>],
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TCow<'a>> {
    if let Some(in_f) = fm.get_func(&cname) {
        return Ok(in_f(&args)?);
    }
    match tm.get_t(cname).map(|t| t.clone()) {
        Ok(in_tp) => {
            let mut v2: Vec<&dyn TParam> = Vec::new();
            for a in args {
                v2.push(a);
            }
            let (s, mut mp) = in_tp.run_exp(&v2, tm, fm)?;
            if let Some(v) = mp.remove("return") {
                return Ok(TCow::Owned(v));
            }
            Ok(TCow::Owned(TData::String(s)))
        }
        Err(e) => e_string(format!("Getting template {}, {}", cname, e)),
    }
}

pub fn run_command<'a, TM: TempManager, FM: FuncManager>(
    cname: &str,
    args: &'a [Expr],
    scope: &'a Scope,
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<TCow<'a>> {
    if cname == "run" {
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
                let (s, mut mp) = t.run_exp(&v, tm, fm)?;
                if let Some(v) = mp.remove("return") {
                    return Ok(TCow::Owned(v));
                }
                return Ok(TCow::Owned(TData::String(s)));
            }
        }
    }
    if cname == "first" {
        for p in args {
            if let Ok(res) = p.run(scope, tm, fm) {
                if *res.deref() != TData::Null {
                    return Ok(res);
                }
            }
        }
        return Ok(TCow::Owned(TData::Null));
    }
    if cname == "select" {
        if args.len() < 3 {
            return e_str("'select' requires at least 1 test and 2 possible values");
        }
        let bval = args[0].run(scope, tm, fm)?;
        let bval = if let Some(n) = bval.as_usize() {
            n + 1
        } else if let Some(b) = bval.as_bool() {
            2 - (b as usize)
        } else {
            return e_string(format!("As Bool failed on {}", bval.deref()));
        };

        if bval > args.len() {
            return e_str("'select' first param must choose return a value less than the length of the rest of the args");
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

impl Expr {
    pub fn run<'a, TM: TempManager, FM: FuncManager>(
        &'a self,
        scope: &'a Scope,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<TCow<'a>> {
        match self {
            Expr::Lit(v) => Ok(TCow::Borrowed(v)),
            Expr::Var(v) => scope.get(v).e_string(format!("No Var by the name {:?}", v)),
            Expr::Command(c, pars) => run_command(c, pars, scope, tm, fm),
            Expr::List(v) => {
                let mut res: Vec<TData> = Vec::new();
                for c in v {
                    let r = c.run(scope, tm, fm)?;
                    res.push(r.into_owned());
                }
                Ok(TCow::Owned(TData::List(res)))
            }
            Expr::Map(m) => {
                let mut res = std::collections::HashMap::new();
                for (k, v) in m {
                    let r = v.run(scope, tm, fm)?;
                    res.insert(k.clone(), r.into_owned());
                }
                Ok(TCow::Owned(TData::Map(res)))
            }
        }
    }
}
