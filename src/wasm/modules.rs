use super::types::*;

pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<Table>,
    mems: Vec<Memory>,
    globals: Vec<Global>,
    elems: Vec<Elem>,
    datas: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Imo>
}

struct ValStack {
    stack: LinkedList<dyn ValType>
}

struct CtrlFrame {
    opcode: u8,
    start_types: Vec<dyn ValType>,
    end_types: Vec<dyn ValType>,
    height: usize,
    unreachable: bool
}