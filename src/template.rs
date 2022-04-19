use crate::*;
use err_tools::*;
use expr::Expr;
use func_man::FuncManager;
use gobble::Parser;
use parse::template::TFile;
use scope::Scope;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use temp_man::{NoTemplates, TempManager};
use tparam::*;

pub type Block = Vec<TreeItem>;

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    pats: Vec<pattern::Pattern>,
    block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreeItem {
    String(String),
    Comment,
    Exp(Expr),
    Block {
        command: String,
        params: Vec<Expr>,
        block: Block,
    },
    If {
        cond: Expr,
        yes: Block,
        no: Option<Block>,
    },
    For {
        k: String,
        v: String,
        p: Expr,
        b: Block,
    },
    Define(String, Vec<String>, Block),
    Global(String, Vec<String>, Block),
    Let(Vec<(String, Expr)>),
    Export(Vec<(String, Expr)>),
    AtLet(String, Block),
    AtExport(String, Block),
    Return(Expr),
    Switch(Vec<Expr>, Vec<Case>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeTemplate {
    pub pnames: Vec<String>,
    pub v: Vec<TreeItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FlatItem {
    String(String),
    Comment,
    Exp(Expr),
    If(Expr),
    Else,
    Elif(Expr),
    For(String, String, Expr),
    AtLet(String),
    AtExport(String),
    Define(String, Vec<String>),
    Global(String, Vec<String>),
    Let(Vec<(String, Expr)>),
    Export(Vec<(String, Expr)>),
    Block(String, Vec<Expr>),
    EndBlock(String),
    Return(Expr),
    Switch(Vec<Expr>),
    Case(Vec<pattern::Pattern>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlatTemplate {
    pub v: Vec<FlatItem>,
}

pub fn run_block<TM: TempManager, FM: FuncManager>(
    block: &Block,
    scope: &mut Scope,
    tm: &mut TM,
    fm: &FM,
) -> anyhow::Result<String> {
    let mut res = String::new();
    scope.push();
    for item in block {
        res.push_str(&item.run(scope, tm, fm)?);
    }
    scope.pop();
    Ok(res)
}

impl TreeItem {
    pub fn run<TM: TempManager, FM: FuncManager>(
        &self,
        scope: &mut Scope,
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<String> {
        match self {
            TreeItem::String(s) => Ok(s.clone()),
            TreeItem::Comment => Ok(String::new()),
            TreeItem::Let(vec) => {
                for (k, v) in vec {
                    let vsolid = v.run(&scope, tm, fm)?.into_owned();
                    scope.set(k.to_string(), vsolid);
                }
                Ok(String::new())
            }
            TreeItem::Export(vec) => {
                for (k, v) in vec {
                    let vsolid = v.run(&scope, tm, fm)?.into_owned();
                    scope.set_root(k.to_string(), vsolid);
                }
                Ok(String::new())
            }
            TreeItem::Return(pipe) => {
                let psolid = pipe.run(&scope, tm, fm)?.into_owned();
                scope.set_root("return".to_string(), psolid);
                Ok(String::new())
            }
            TreeItem::Exp(p) => {
                let pres = &p.run(&scope, tm, fm)?;
                Ok(pres.to_string())
            }
            TreeItem::If { cond, yes, no } => {
                let pres = cond.run(&scope, tm, fm).unwrap_or(TCow::Owned(TData::Null));
                match pres.as_bool() {
                    Some(true) => run_block(yes, scope, tm, fm),
                    _ => {
                        if let Some(n) = no {
                            run_block(n, scope, tm, fm)
                        } else {
                            Ok(String::new())
                        }
                    }
                }
            }
            TreeItem::For { k, v, p, b } => {
                let looper = p.run(scope, tm, fm)?.into_owned();
                let mut res = String::new();
                match looper {
                    TData::Map(m) => {
                        for (mapk, mapv) in m {
                            scope.set(k, TData::String(mapk.to_string()));
                            scope.set(v, mapv.clone());
                            res.push_str(&run_block(&b, scope, tm, fm)?);
                        }
                        Ok(res)
                    }
                    TData::List(l) => {
                        for (listn, listv) in l.iter().enumerate() {
                            scope.set(k, TData::UInt(listn));
                            scope.set(v, listv.clone());
                            res.push_str(&run_block(&b, scope, tm, fm)?);
                        }
                        Ok(res)
                    }
                    TData::UInt(n) => {
                        for (listn, listv) in (0..n).enumerate() {
                            scope.set(k, TData::UInt(listn));
                            scope.set(v, TData::UInt(listv));
                            res.push_str(&run_block(&b, scope, tm, fm)?);
                        }
                        Ok(res)
                    }
                    TData::Int(n) => {
                        for (listn, listv) in (0..n).enumerate() {
                            scope.set(k, TData::UInt(listn));
                            scope.set(v, TData::Int(listv));
                            res.push_str(&run_block(&b, scope, tm, fm)?);
                        }
                        Ok(res)
                    }
                    _ => e_str("Cannot loop over non map or list"),
                }
            }
            TreeItem::Block {
                command,
                params,
                block,
            } => {
                let ch = run_block(block, scope, tm, fm)?;
                if params.len() == 0 {
                    return Ok(expr::run_values::<TM, FM>(
                        command,
                        &vec![TCow::Owned(TData::String(ch))],
                        tm,
                        fm,
                    )?
                    .to_string());
                }
                scope.set("@", TData::String(ch));
                let mut v = vec![];
                for p in params {
                    v.push(p.run(scope, tm, fm)?);
                }
                Ok(expr::run_values::<TM, FM>(command, &v, tm, fm)?.to_string())
            }
            TreeItem::AtLet(name, block) => {
                let ch = run_block(block, scope, tm, fm)?;
                scope.set(name, TData::String(ch));
                Ok(String::new())
            }
            TreeItem::AtExport(name, block) => {
                let ch = run_block(block, scope, tm, fm)?;
                scope.set_root(name, TData::String(ch));
                Ok(String::new())
            }
            TreeItem::Define(name, pnames, block) => {
                scope.set(
                    name,
                    TData::Template(TreeTemplate {
                        v: block.clone(),
                        pnames: pnames.clone(),
                    }),
                );
                Ok(String::new())
            }
            TreeItem::Global(name, pnames, block) => {
                tm.insert_t(
                    name.to_string(),
                    TreeTemplate {
                        v: block.clone(),
                        pnames: pnames.clone(),
                    },
                );
                Ok(String::new())
            }
            TreeItem::Switch(params, cases) => {
                let mut s_params = Vec::new();
                for p in params {
                    match p.run(scope, tm, fm) {
                        Ok(v) => s_params.push(v.into_owned()),
                        Err(_) => break,
                    }
                }
                'caseloop: for c in cases {
                    scope.push();
                    for (n, p) in c.pats.iter().enumerate() {
                        match s_params.get(n) {
                            Some(b) => {
                                if !p.match_data(b, scope, tm, fm) {
                                    scope.pop();
                                    continue 'caseloop;
                                }
                            }
                            None => {
                                scope.pop();
                                continue 'caseloop;
                            }
                        }
                    }
                    let r = run_block(&c.block, scope, tm, fm);
                    scope.pop();
                    return r;
                }
                return Ok(String::new());
            }
        }
    }
}

impl TreeTemplate {
    pub fn run<TM: TempManager, FM: FuncManager>(
        &self,
        params: &[&dyn TParam],
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<String> {
        self.run_exp(params, tm, fm).map(|(s, _)| s)
    }

    pub fn run_exp<TM: TempManager, FM: FuncManager>(
        &self,
        params: &[&dyn TParam],
        tm: &mut TM,
        fm: &FM,
    ) -> anyhow::Result<(String, HashMap<String, TData>)> {
        let mut res = String::new();
        let mut scope = Scope::new(params);
        let mut it = (&self.v).into_iter();
        while let Some(item) = it.next() {
            res.push_str(&item.run(&mut scope, tm, fm)?);
        }
        Ok((res, scope.top()))
    }

    //It is not considered a failure if a file has no front matter
    pub fn front_matter<FM: FuncManager>(&self, fm: &FM) -> HashMap<String, TData> {
        let mut scope = Scope::new(&[]);
        let mut it = (&self.v).into_iter();
        while let Some(item) = it.next() {
            match item {
                TreeItem::AtExport(name, block) => {
                    match run_block(block, &mut scope, &mut NoTemplates, fm) {
                        Ok(val) => scope.set_root(name, TData::String(val)),
                        Err(_) => {}
                    }
                }
                TreeItem::Export(vec) => {
                    for (k, v) in vec {
                        match v.run(&scope, &mut NoTemplates, fm).map(|v| v.into_owned()) {
                            Ok(val) => scope.set_root(k.to_string(), val),
                            Err(_) => {}
                        }
                    }
                }
                _ => {}
            }
        }
        scope.top()
    }

    pub fn load<P: AsRef<Path>>(p: P) -> anyhow::Result<Self> {
        let mut f = std::fs::File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(TreeTemplate::from_str(&s)?)
    }
}

/// Handles all openers, but not any of the closers
pub fn tt_basic<I: Iterator<Item = FlatItem>>(
    fi: FlatItem,
    it: &mut I,
) -> anyhow::Result<TreeItem> {
    Ok(match fi {
        FlatItem::String(s) => TreeItem::String(s),
        FlatItem::Exp(p) => TreeItem::Exp(p),
        FlatItem::Let(v) => TreeItem::Let(v),
        FlatItem::AtLet(v) => TreeItem::AtLet(v, tt_name_block("let", it)?),
        FlatItem::Export(v) => TreeItem::Export(v),
        FlatItem::AtExport(v) => TreeItem::AtExport(v, tt_name_block("export", it)?),

        FlatItem::Define(v, params) => TreeItem::Define(v, params, tt_name_block("define", it)?),
        FlatItem::Global(v, params) => TreeItem::Global(v, params, tt_name_block("global", it)?),
        FlatItem::If(p) => tt_if_yes(p, it)?,
        FlatItem::For(k, v, p) => TreeItem::For {
            k,
            v,
            p,
            b: tt_for(it)?,
        },
        FlatItem::Block(command, params) => {
            let block = tt_name_block(&command, it)?;
            TreeItem::Block {
                command,
                params,
                block,
            }
        }
        FlatItem::Return(p) => TreeItem::Return(p),
        FlatItem::Comment => TreeItem::Comment,
        FlatItem::Switch(p) => return tt_switch(p, it),
        e => return e_string(format!("Unexpected {:?}", e)),
    })
}
pub fn tt_switch<I: Iterator<Item = FlatItem>>(
    params: Vec<Expr>,
    it: &mut I,
) -> anyhow::Result<TreeItem> {
    let mut res = Vec::new();
    let mut curr = None;
    while let Some(t) = it.next() {
        match t {
            FlatItem::Case(pats) => {
                curr.take().map(|n| res.push(n));
                curr = Some(Case {
                    pats,
                    block: Vec::new(),
                });
            }
            FlatItem::String(s) => match curr {
                Some(ref mut cb) => cb.block.push(TreeItem::String(s)),
                None => {}
            },
            FlatItem::EndBlock(s) if s == "switch" => {
                curr.take().map(|n| res.push(n));
                return Ok(TreeItem::Switch(params, res));
            }
            v => match curr {
                Some(ref mut cb) => cb.block.push(tt_basic(v, it)?),
                None => return e_str("Switch requires cases to add blocks to"),
            },
        }
    }
    curr.take().map(|n| res.push(n));
    Ok(TreeItem::Switch(params, res))
}

pub fn tt_root_block<I: Iterator<Item = FlatItem>>(i: &mut I) -> anyhow::Result<TreeTemplate> {
    let mut res = Vec::new();
    while let Some(t) = i.next() {
        res.push(tt_basic(t, i)?)
    }
    Ok(TreeTemplate {
        v: res,
        pnames: Vec::new(),
    })
}

pub fn tt_name_block<I: Iterator<Item = FlatItem>>(name: &str, i: &mut I) -> anyhow::Result<Block> {
    let mut res = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::EndBlock(n) if n == name => return Ok(res),
            other => res.push(tt_basic(other, i)?),
        }
    }
    Ok(res)
    // Should I allow open stuff at the end of a file? @md says yes
    //Err(Error::String(format!("{} block not ended", name)))
}

pub fn tt_if_yes<I: Iterator<Item = FlatItem>>(cond: Expr, it: &mut I) -> anyhow::Result<TreeItem> {
    let mut yes = Vec::new();
    while let Some(t) = it.next() {
        match t {
            FlatItem::Else => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: tt_else(it)?,
                })
            }
            FlatItem::Elif(p) => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: Some(vec![tt_if_yes(p, it)?]),
                })
            }
            FlatItem::EndBlock(v) if v == "if" => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: None,
                })
            }
            other => yes.push(tt_basic(other, it)?),
        }
    }
    //Should this fail?
    e_str("Expected '/if' 'else' or 'elif'")
}

pub fn tt_else<I: Iterator<Item = FlatItem>>(it: &mut I) -> anyhow::Result<Option<Block>> {
    let mut no = Vec::new();
    while let Some(t) = it.next() {
        match t {
            FlatItem::EndBlock(v) if v == "if" => return Ok(Some(no)),
            other => no.push(tt_basic(other, it)?),
        }
    }
    Ok(Some(no))
}

pub fn tt_for<I: Iterator<Item = FlatItem>>(i: &mut I) -> anyhow::Result<Block> {
    let mut block = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::EndBlock(v) if v == "for" => return Ok(block),
            other => block.push(tt_basic(other, i)?),
        }
    }
    Ok(block)
}

impl FlatTemplate {
    pub fn to_tree(self) -> anyhow::Result<TreeTemplate> {
        tt_root_block(&mut self.v.into_iter())
    }
}

impl std::str::FromStr for TreeTemplate {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flat = TFile.parse_s(s).map_err(|e| e.strung())?;
        flat.to_tree().map_err(|e| e.into())
    }
}
impl std::str::FromStr for FlatTemplate {
    type Err = gobble::StrungError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TFile.parse_s(s).map_err(|e| e.strung())
    }
}
