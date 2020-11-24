pub mod expr;
pub mod tdata;
pub mod template;

use gobble::*;

char_bool!(WN, " \t\n\r");

pub fn wn_<P: Parser>(p: P) -> impl Parser<Out = P::Out> {
    last(WN.star(), p)
}
pub fn wn__<P: Parser>(p: P) -> impl Parser<Out = P::Out> {
    middle(WN.star(), p, WN.star())
}
