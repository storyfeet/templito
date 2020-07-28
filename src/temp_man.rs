use crate::*;
use std::collections::HashMap;

pub type BasicTemps = HashMap<String, TreeTemplate>;

pub trait TempManager {
    fn insert_t(&mut self, k: String, t: TreeTemplate);
    fn get_t(&mut self, k: &str) -> Option<&TreeTemplate>;
}

impl TempManager for BasicTemps {
    fn insert_t(&mut self, k: String, t: TreeTemplate) {
        self.insert(k, t);
    }
    fn get_t(&mut self, k: &str) -> Option<&TreeTemplate> {
        self.get(k)
    }
}
