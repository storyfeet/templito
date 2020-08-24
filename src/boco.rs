//! Borrow or concrete
//!

use std::fmt;
use std::ops::Deref;

pub type Bop<'a, T> = Option<Boco<'a, T>>;

#[derive(Clone)]
pub enum Boco<'a, T> {
    Bo(&'a T),
    Co(T),
}

impl<'a, T: fmt::Debug> fmt::Debug for Boco<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<'a, T> std::ops::Deref for Boco<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Boco::Co(c) => c,
            Boco::Bo(b) => *b,
        }
    }
}
impl<'a, T: Clone> Boco<'a, T> {
    pub fn concrete(self) -> T {
        match self {
            Boco::Bo(b) => b.clone(),
            Boco::Co(c) => c,
        }
    }
}

pub fn boco_c<'a, T>(t: T) -> Bop<'a, T> {
    Some(Boco::Co(t))
}
pub fn boco_b<'a, T>(t: &'a T) -> Bop<'a, T> {
    Some(Boco::Bo(t))
}

pub fn b_ok<'a, T, E>(t: T) -> Result<Boco<'a, T>, E> {
    Ok(Boco::Co(t))
}
