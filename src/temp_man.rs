use crate::*;
use err::*;
use std::collections::HashMap;

pub type BasicTemps = HashMap<String, TreeTemplate>;

pub trait TempManager {
    fn insert_t(&mut self, k: String, t: TreeTemplate);
    fn get_t(&mut self, k: &str) -> anyhow::Result<&TreeTemplate>;
}

impl TempManager for BasicTemps {
    fn insert_t(&mut self, k: String, t: TreeTemplate) {
        self.insert(k, t);
    }
    fn get_t(&mut self, k: &str) -> anyhow::Result<&TreeTemplate> {
        self.get(k).ok_or(ea_str("Template not found"))
    }
}

pub struct NoTemplates;

impl TempManager for NoTemplates {
    fn insert_t(&mut self, _k: String, _t: TreeTemplate) {}
    fn get_t(&mut self, _k: &str) -> anyhow::Result<&TreeTemplate> {
        Err(ea_str("No Templates on \"NoTemplates\" type"))
    }
}
