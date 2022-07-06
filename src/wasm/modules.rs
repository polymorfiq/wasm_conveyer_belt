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

struct CtrlFrame {
    opcode: u8,
    start_types: Vec<ValType>,
    end_types: Vec<ValType>,
    height: usize,
    unreachable: bool
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