use crate::*;
use err::*;
use gobble::Parser;
use parser::TFile;
use pipeline::*;
use scope::Scope;

pub type Block = Vec<TreeItem>;

#[derive(Clone, Debug, PartialEq)]
pub enum TreeItem {
    String(String),
    Pipe(Pipeline),
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
    Let(Vec<(String, Pipeline)>),
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
impl TreeTemplate {
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
                TreeItem::String(s) => res.push_str(&s),
                TreeItem::Let(vec) => {
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

pub fn flat_basic(fi: FlatItem) -> Result<TreeItem, Error> {
    Ok(match fi {
        FlatItem::String(s) => TreeItem::String(s),
        FlatItem::Pipe(p) => TreeItem::Pipe(p),
        FlatItem::Let(v) => TreeItem::Let(v),
        e => return Err(Error::String(format!("Unexpected {:?}", e))),
    })
}

pub fn tt_block<I: Iterator<Item = FlatItem>>(i: &mut I) -> Result<TreeTemplate, Error> {
    let mut res = Vec::new();
    while let Some(t) = i.next() {
        res.push(match t {
            FlatItem::If(p) => tt_if_yes(p, i)?,
            FlatItem::For(k, v, p) => TreeItem::For {
                k,
                v,
                p,
                b: tt_for(i)?,
            },
            other => flat_basic(other)?,
        })
    }
    Ok(TreeTemplate { v: res })
}

pub fn tt_if_yes<I: Iterator<Item = FlatItem>>(
    cond: Pipeline,
    i: &mut I,
) -> Result<TreeItem, Error> {
    let mut yes = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::If(p) => yes.push(tt_if_yes(p, i)?), //TODO
            FlatItem::For(k, v, p) => yes.push(TreeItem::For {
                k,
                v,
                p,
                b: tt_for(i)?,
            }),
            FlatItem::Else => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: tt_if_no(i)?,
                })
            }
            FlatItem::Elif(p) => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: Some(vec![tt_if_yes(p, i)?]),
                })
            }
            FlatItem::EndIf => {
                return Ok(TreeItem::If {
                    cond,
                    yes,
                    no: None,
                })
            }
            other => yes.push(flat_basic(other)?),
        }
    }
    //Should this fail?
    Err(Error::Str("Expected '/if' 'else' or 'elif'"))
}

pub fn tt_if_no<I: Iterator<Item = FlatItem>>(i: &mut I) -> Result<Option<Block>, Error> {
    let mut no = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::If(p) => no.push(tt_if_yes(p, i)?), //TODO
            FlatItem::For(k, v, p) => no.push(TreeItem::For {
                k,
                v,
                p,
                b: tt_for(i)?,
            }),
            FlatItem::EndIf => return Ok(Some(no)),
            other => no.push(flat_basic(other)?),
        }
    }
    Ok(Some(no))
}

pub fn tt_for<I: Iterator<Item = FlatItem>>(i: &mut I) -> Result<Block, Error> {
    let mut block = Vec::new();
    while let Some(t) = i.next() {
        match t {
            FlatItem::If(p) => block.push(tt_if_yes(p, i)?), //TODO
            FlatItem::For(k, v, p) => block.push(TreeItem::For {
                k,
                v,
                p,
                b: tt_for(i)?,
            }),
            FlatItem::EndFor => return Ok(block),
            other => block.push(flat_basic(other)?),
        }
    }
    Ok(block)
}

impl FlatTemplate {
    pub fn to_tree(self) -> Result<TreeTemplate, Error> {
        tt_block(&mut self.v.into_iter())
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
