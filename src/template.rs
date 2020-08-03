use crate::*;
use err::*;
use func_man::FuncManager;
use gobble::Parser;
use parser::TFile;
use pipeline::*;
use scope::Scope;
use std::collections::HashMap;
use temp_man::TempManager;
use tparam::TParam;

pub type Block = Vec<TreeItem>;

#[derive(Clone, Debug, PartialEq)]
pub enum TreeItem {
    String(String),
    Pipe(Pipeline),
    Block {
        command: String,
        params: Vec<Pipeline>,
        block: Block,
    },
    If {
        cond: Pipeline,
        yes: Block,
        no: Option<Block>,
    },
    For {
        k: String,
        v: String,
        p: Pipeline,
        b: Block,
    },
    Let(Vec<(String, Pipeline)>),
    Export(Vec<(String, Pipeline)>),
    AtLet(String, Block),
    AtExport(String, Block),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeTemplate {
    pub v: Vec<TreeItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FlatItem {
    String(String),
    Pipe(Pipeline),
    If(Pipeline),
    Else,
    Elif(Pipeline),
    For(String, String, Pipeline),
    AtLet(String),
    AtExport(String),
    Let(Vec<(String, Pipeline)>),
    Export(Vec<(String, Pipeline)>),
    Block(String, Vec<Pipeline>),
    EndBlock(String),
    EndIf,
    EndFor,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VarPart {
    Num(usize),
    Id(String),
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
            TreeItem::Let(vec) => {
                for (k, v) in vec {
                    let vsolid = v.run(&scope, tm, fm)?;
                    scope.set(k.to_string(), vsolid);
                }
                Ok(String::new())
            }
            TreeItem::Export(vec) => {
                for (k, v) in vec {
                    let vsolid = v.run(&scope, tm, fm)?;
                    scope.set_root(k.to_string(), vsolid);
                }
                Ok(String::new())
            }
            TreeItem::Pipe(p) => {
                let pres = &p.run(&scope, tm, fm)?;
                Ok(pres.to_string())
            }
            TreeItem::If { cond, yes, no } => {
                let pres = cond.run(&scope, tm, fm)?;
                match pres.as_bool() {
                    Some(true) => run_block(yes, scope, tm, fm),
                    Some(false) => {
                        if let Some(n) = no {
                            run_block(n, scope, tm, fm)
                        } else {
                            Ok(String::new())
                        }
                    }
                    None => {
                        return Err(Error::String(format!("Cannot treat {:?} as Bool", pres)).into())
                    }
                }
            }
            TreeItem::For { k, v, p, b } => {
                let looper = p.run(scope, tm, fm)?;
                let mut res = String::new();
                if let Some(keys) = looper.keys() {
                    for kname in keys {
                        let vval = looper.get_key(&kname).ok_or(Error::Str("Key Missing"))?;
                        scope.set(k, TData::String(kname));
                        scope.set(v, vval.clone());
                        res.push_str(&run_block(&b, scope, tm, fm)?);
                    }
                    Ok(res)
                } else if let Some(len) = looper.len() {
                    for pos in 0..len {
                        let vval = looper.get_index(pos).ok_or(Error::Str("Key Missing"))?;
                        scope.set(k, TData::UInt(pos));
                        scope.set(v, vval.clone());
                        res.push_str(&run_block(&b, scope, tm, fm)?);
                    }
                    Ok(res)
                } else {
                    //TODO try range object, Not sure how to handle this
                    Err(Error::String(format!("Cannot loop on {:?}", looper)).into())
                }
            }
            TreeItem::Block {
                command,
                params,
                block,
            } => {
                let ch = run_block(block, scope, tm, fm)?;
                scope.set("@", TData::String(ch));
                let mut v = vec![];
                for p in params {
                    v.push(p.run(scope, tm, fm)?);
                }
                Ok(pipeline::run_values::<TM, FM>(command, &v, tm, fm)?.to_string())
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
}

///Handles all openers, but not any of the closers
pub fn tt_basic<I: Iterator<Item = FlatItem>>(fi: FlatItem, it: &mut I) -> Result<TreeItem, Error> {
    Ok(match fi {
        FlatItem::String(s) => TreeItem::String(s),
        FlatItem::Pipe(p) => TreeItem::Pipe(p),
        FlatItem::Let(v) => TreeItem::Let(v),
        FlatItem::AtLet(v) => TreeItem::AtLet(v, tt_name_block("let", it)?),
        FlatItem::Export(v) => TreeItem::Export(v),
        FlatItem::AtExport(v) => TreeItem::AtExport(v, tt_name_block("let", it)?),
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
        e => return Err(Error::String(format!("Unexpected {:?}", e))),
    })
}

pub fn tt_root_block<I: Iterator<Item = FlatItem>>(i: &mut I) -> Result<TreeTemplate, Error> {
    let mut res = Vec::new();
    while let Some(t) = i.next() {
        res.push(tt_basic(t, i)?)
    }
    Ok(TreeTemplate { v: res })
}

pub fn tt_name_block<I: Iterator<Item = FlatItem>>(name: &str, i: &mut I) -> Result<Block, Error> {
    let mut res = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::EndBlock(n) if n == name => return Ok(res),
            other => res.push(tt_basic(other, i)?),
        }
    }
    Err(Error::String(format!("{} block not ended", name)))
}

pub fn tt_if_yes<I: Iterator<Item = FlatItem>>(
    cond: Pipeline,
    it: &mut I,
) -> Result<TreeItem, Error> {
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
            FlatItem::EndIf => {
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
    Err(Error::Str("Expected '/if' 'else' or 'elif'"))
}

pub fn tt_else<I: Iterator<Item = FlatItem>>(it: &mut I) -> Result<Option<Block>, Error> {
    let mut no = Vec::new();
    while let Some(t) = it.next() {
        match t {
            FlatItem::EndIf => return Ok(Some(no)),
            other => no.push(tt_basic(other, it)?),
        }
    }
    Ok(Some(no))
}

pub fn tt_for<I: Iterator<Item = FlatItem>>(i: &mut I) -> Result<Block, Error> {
    let mut block = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::EndFor => return Ok(block),
            other => block.push(tt_basic(other, i)?),
        }
    }
    Ok(block)
}

impl FlatTemplate {
    pub fn to_tree(self) -> Result<TreeTemplate, Error> {
        tt_root_block(&mut self.v.into_iter())
    }
}

impl std::str::FromStr for TreeTemplate {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flat = TFile.parse_s(s).map_err(|e| e.strung(s.to_string()))?;
        flat.to_tree().map_err(|e| e.into())
    }
}
impl std::str::FromStr for FlatTemplate {
    type Err = gobble::StrungError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TFile.parse_s(s).map_err(|e| e.strung(s.to_string()))
    }
}
