use super::instructions::*;

pub(super) struct Expr<'a> {a: Vec<&'a dyn Instr>}