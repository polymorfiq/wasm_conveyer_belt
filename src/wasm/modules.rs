use super::types::*;

pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<TableType>,
    mems: Vec<MemType>,
    globals: Vec<Global>,
    elems: Vec<Elem>,
    datas: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            types: Vec::new(),
            funcs: Vec::new(),
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            elems: Vec::new(),
            datas: Vec::new(),
            start: None,
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }

    pub fn get_block_type(&self, blocktype: BlockType) -> FuncType {
        match blocktype {
            BlockType::TypeIdx(typeidx) => vec![self.types[typeidx]],
            BlockType::ValType(None) => FuncType{inputs: Vec::new(), returns: Vec::new()},
            BlockType::ValType(Some(valtype)) => FuncType {
                inputs: vec![valtype],
                returns: vec![valtype]
            }
        }
        
    }
}

struct CtrlFrame {
    opcode: u8,
    start_types: Vec<ValType>,
    end_types: Vec<ValType>,
    height: usize,
    unreachable: bool
}

pub(super) struct Context {
    pub types: Vec<FuncType>,
    pub funcs: Vec<FuncType>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<GlobalType>,
    pub elems: Vec<RefType>,
    pub datas: Vec<u8>,
    pub locals: Vec<ValType>,
    pub labels: Vec<ResultType>,
    pub returns: Option<ResultType>,
    pub refs: Vec<FuncIdx>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            types: vec![],
            funcs: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
            locals: vec![],
            labels: vec![],
            returns: vec![],
            refs: vec![],
        }
    }
}