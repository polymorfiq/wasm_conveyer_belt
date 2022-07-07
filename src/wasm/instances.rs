use super::types::*;
use super::instructions::*;

pub(super) enum FuncInst {
    Func(FuncType, ModuleInst, Func),
    Host(FuncType, HostFunc)
}

type HostFunc = u32;

pub(super) struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcsaddrs: Vec<FuncAddr>,
    pub tableaddrs: Vec<TableAddr>,
    pub memaddrs: Vec<MemAddr>,
    pub globaladdrs: Vec<GlobalAddr>,
    pub elemaddrs: Vec<ElemAddr>,
    pub dataaddrs: Vec<DataAddr>,
    pub exports: Vec<ExportInst>
}

pub(super) struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>
}

pub(super) struct TableInst {
    pub _type: TableType,
    pub elem: Vec<Val>,
}

pub (super) struct MemInst {
    pub _type: MemType,
    pub data: Vec<Byte>
}

pub (super) struct GlobalInst {
    pub _type: GlobalType,
    pub value: Val
}

pub(super) struct ElemInst {
    pub _type: ValType,
    pub elem: Vec<Val>
}

pub(super) struct DataInst {
    pub data: Vec<Byte>
}

pub(super) struct ExportInst {
    pub name: String,
    pub value: ExternVal
}

pub(super) enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr)
}

// Stack types

pub(super) struct Label {
    arity: u32,
    instrs: Vec<Instr>
}

pub(super) struct Activation<'a> {
    arity: u32,
    frame: Frame<'a>
}

pub(super) struct Frame<'a> {
    locals: Vec<Val>,
    module: &'a mut ModuleInst
}

impl Frame<'_> {
    pub fn expand(&self, blocktype: BlockType) -> FuncType {
        match blocktype {
            BlockType::TypeIdx(typeidx) => vec![self.module.types[typeidx]],
            BlockType::ValType(None) => FuncType{inputs: vec![], returns: vec![]},
            BlockType::ValType(Some(valtype)) => FuncType {
                inputs: vec![],
                returns: vec![valtype]
            }
        }
        
    }
}

// Configurations
pub(super) struct Config<'a> {
    pub store: Store,
    pub thread: Thread<'a>
}

pub(super) struct Thread<'a> {
    pub frame: Frame<'a>,
    pub instr: Vec<Instr>
}