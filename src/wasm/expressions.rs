use super::instructions::*;

pub(super) struct Expr<'a> {instrs: &'a [&'a dyn Instr]}