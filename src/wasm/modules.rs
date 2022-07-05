use super::types::*;

pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<Table>,
    mems: Vec<Memory>,
    globals: Vec<Global>,
    elems: Vec<Elem>,
    data: Vec<Data>,
}