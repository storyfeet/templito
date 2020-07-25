use crate::*;
use gobble::Parser;
use parser::TFile;
use pipeline::*;
use scope::Scope;

#[derive(Clone, Debug)]
pub struct Block(Vec<TreeItem>);

#[derive(Clone, Debug)]
pub enum TreeItem {
    String(String),
    Pipe(Pipeline),
    If {
        cond: Pipeline,
        yes: Block,
        No: Option<Block>,
    },
    For(String, String, Pipeline),
    Let(Vec<(String, Pipeline)>),
}

#[derive(Clone)]
pub struct TreeTemplate {
    pub v: Vec<TreeItem>,
}

#[derive(Clone,Debug)]
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

#[derive(Clone, Debug)]
pub enum VarPart {
    Num(usize),
    Id(String),
}

#[derive(Clone)]
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

pub fn flat_basic(fi:FlatItem)->Result<TreeItem,err::Error>{
    Ok(match fi {
        FlatItem::String(s)=>TreeItem::String(s),
        FlatItem::Pipe(p)=>TreeItem::Pipe(p),
        FlatItem::Let(v)=>TreeItem::Let(v),
        e=>return Err(err::Error::String(format!("Unexpected {:?}",e))),
    })
}

pub fn tt_block<I:Iterator<Item=FlatItem>>(i:&mut I)->Result<TreeTemplate,err::Error>{
    let mut res = Vec::new();    
    while let Some(t) =i.next(){
        res.push(match t {
        FlatItem::If(p)=>tt_if_yes(p,i)?,
        FlatItem::For(k, v, p)=>tt_if_yes(p,i)?,//TODO
        other=>flat_basic(other)?,
        })
    }
    Ok(TreeTemplate{v:res})
}

pub fn tt_if_yes<I:Iterator<Item=FlatItem>>(cond:Pipeline,i:&mut I)->Result<TreeItem,err::Error>{
    let mut yes = Vec::new();    
    while let Some(t) =i.next(){
        match t {
            FlatItem::If(p)=>yes.push(tt_if_yes(p,i)?),//TODO
            FlatItem::For(k, v, p)=>{},//TODO
            other=>yes.push(flat_basic(other)?),
        }
    }
    Ok(TreeItem::If{yes})
}

pub fn tt_if_no<I:Iterator<Item=FlatItem>>(i:&mut I)->



impl FlatTemplate {
    pub fn to_tree(self) -> Result<TreeTemplate, err::Error> {
        let it =  
    }
}

impl std::str::FromStr for FlatTemplate {
    type Err = gobble::StrungError;
    fn from_str(s: &str) -> Result<FlatTemplate, Self::Err> {
        TFile.parse_s(s).map_err(|e| e.strung(s.to_string()))
    }
}
