pub mod err;
pub mod func_man;
pub mod funcs;
pub mod impls;
mod parser;
mod pipeline;
mod scope;
pub mod temp_man;
pub mod template;
mod tests;
use template::{TreeTemplate, VarPart};
pub mod prelude;

use std::fmt::{Debug, Display};

pub trait Templable: 'static + Sized + PartialEq + Debug + Display + Clone {
    ///How the type will be created from the template
    fn parse_lit(s: &str) -> anyhow::Result<Self>;
    ///Create a string instance of the type,
    /// eg for serde_json 'Some(Value::String(s))'
    fn string(s: &str) -> Self;
    ///Check if this is a string type and return that
    ///Should not normally convert other types to string
    fn as_str(&self) -> Option<&str> {
        None
    }

    ///No need to override this one, It tries as_str first otherwise uses Display
    fn string_it(&self) -> String {
        match self.as_str() {
            Some(s) => s.to_string(),
            None => self.to_string(),
        }
    }

    ///Create the type from a bool value
    ///This will be the result of boolean operations in the template
    fn bool(b: bool) -> Self;

    ///If this type has a sensible bool value return that.
    ///Will be used for binary logic
    fn as_bool(&self) -> Option<bool> {
        None
    }

    ///This is not a None/Null type
    fn is_valid(&self) -> bool {
        true
    }

    ///Create this object from a usize or index value,
    ///This will be the key value in for loops over arrays,
    fn usize(u: usize) -> Self;

    ///Return the usize value that will be used for lookups and indexing
    fn as_usize(&self) -> Option<usize> {
        None
    }

    ///Return a list of keys for for loops combined with
    fn keys(&self) -> Option<Vec<String>> {
        None
    }

    ///The len will be used in for loops when the value can be treated as an array
    ///Return None if the item cannot be indexed like an array
    fn len(&self) -> Option<usize> {
        None
    }
    ///This is used with 'keys' by for loops
    fn get_key<'a>(&'a self, _s: &str) -> Option<&'a Self> {
        None
    }
    ///This is used by for loops
    fn get_index<'a>(&'a self, _n: usize) -> Option<&'a Self> {
        None
    }
    ///This will be given a function name for you to return the function, This will be the
    ///commands you wish to implement that work specifically for this type.
    ///func_manager will be asked, then the template_manager If all of these fail there will
    ///be an error
    ///For example Map/Array creating, or maths that this type might handle differently
    ///For an example look at crate::impls::json.rs
    fn get_func(_s: &str) -> Option<&'static func_man::TFunc<Self>> {
        None
    }

    ///This function exists to enable '$0.cat.3' indexing
    fn get_var_part<'a>(&'a self, v: &VarPart) -> Option<&'a Self> {
        match v {
            VarPart::Num(n) => self.get_index(*n),
            VarPart::Id(s) => self.get_key(s),
        }
    }

    fn get_var_path<'a>(&'a self, v: &[VarPart]) -> Option<&'a Self> {
        if v.len() == 0 {
            return Some(self);
        }
        self.get_var_part(&v[0])
            .and_then(|p| p.get_var_path(&v[1..]))
    }

    fn compare(&self, b: &Self) -> Option<std::cmp::Ordering>;
    fn list(_v: Vec<Self>) -> Option<Self> {
        None
    }
    //TODO consider having an path->Self method that's auto implemented
}
