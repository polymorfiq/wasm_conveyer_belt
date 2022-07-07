use super::instructions::Expr;

#[derive(Clone, Copy, std::fmt::Debug)]
pub(super) enum ValType {
    // Numeric
    AnyNum,
    I32,
    I64,
    F32,
    F64,
    V128,

    // Refs
    FuncRef,
    ExternRef,
    AnyRef,

    // Any
    Unknown
}
impl ValType {
    pub fn is_num(self) -> bool {
        match self {
            ValType::AnyNum => true,
            ValType::I32 => true,
            ValType::I64 => true,
            ValType::F32 => true,
            ValType::F64 => true,
            ValType::Unknown => true,
            _ => false
        }
    }

    pub fn is_vec(self) -> bool {
        match self {
            ValType::V128 => true,
            ValType::Unknown => true,
            _ => false
        }
    }

    pub fn is_ref(self) -> bool {
        match self {
            ValType::AnyRef => true,
            ValType::FuncRef => true,
            ValType::ExternRef => true,
            ValType::Unknown => true,
            _ => false
        }
    }
}

pub(super) trait Val {
    fn is_type(&self, t: ValType) -> bool;
}

impl Val for dyn NumType {
    fn is_type(&self, t: ValType) -> bool {
        match t {
            ValType::AnyNum => true,
            ValType::Unknown => true,
            _ => self.is_type(t)
        }
    }
}
impl Val for dyn VecType {
    fn is_type(&self, t: ValType) -> bool {
        match t {
            ValType::Unknown => true,
            _ => self.is_type(t)
        }
    }
}

pub(super) type ResultType = Vec<ValType>;

// Num Types
pub(super) trait NumType {
    fn is_type(&self, t: ValType) -> bool;
}
impl NumType for I32 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::I32) }
}
impl NumType for I64 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::I64) }
}
impl NumType for F32 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::F32) }
}
impl NumType for F64 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::F64) }
}

pub(super) struct I8 {data: u8}
pub(super) struct I16 {data: u16}
pub(super) struct I32 {data: u32}
pub(super) struct I64 {data: u64}
pub(super) struct F32 {data: f32}
pub(super) struct F64 {data: f64}

// Vector Types
pub(super) trait VecType {
    fn is_type(&self, t: ValType) -> bool;
    fn bits(&self) -> u128;
}

impl VecType for V128 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::V128) }
    fn bits(&self) -> u128 { self.data }
}

pub(super) struct V128 {data: u128}

// Index Types
pub(super) type Byte = u8;
pub(super) type LaneIdx = u8;
pub(super) type TypeIdx = usize;
pub(super) type FuncIdx = usize;
pub(super) type TableIdx = usize;
pub(super) type MemIdx = usize;
pub(super) type GlobalIdx = usize;
pub(super) type ElemIdx = usize;
pub(super) type DataIdx = usize;
pub(super) type LocalIdx = usize;
pub(super) type LabelIdx = usize;
pub(super) type Offset = usize;
pub(super) type Align = usize;

#[derive(std::fmt::Debug)]
pub(super) enum BlockType {
    TypeIdx(TypeIdx),
    ValType(Option<ValType>)
}

// Limits
pub(super) struct Limits {
    min: u32,
    max: Option<u32>,
}

// Memory Types
pub(super) struct MemType {limits: Limits}

#[derive(std::fmt::Debug)]
pub(super) struct MemArg {
    pub offset: u32,
    pub align: u32
}

// Function Types
pub(super) struct Start {
    pub func: FuncIdx
}

pub(super) struct FuncType {pub inputs: Vec<ValType>, pub returns: Vec<ValType>}

pub(super) struct Func {
    func_type: TypeIdx,
    locals: Vec<ValType>,
    body: Expr
}

// Table Types
pub(super) struct TableType {pub limits: Limits, pub reftype: ValType}

pub(super) struct Elem {reftype: ValType, init: Vec<Expr>, mode: ElemMode}
pub(super) enum ElemMode {
    Passive,
    Active(TableIdx, Offset),
    Declarative
}

// Data Types
pub(super) struct Data {
    init: Vec<Byte>,
    mode: DataMode
}

pub(super) enum DataMode {
    Passive,
    Active(MemIdx, Offset),
}

// Global Types
pub(super) enum Mut {
    Var,
    Const
}

pub(super) struct GlobalType {pub valmut: Mut, pub valtype: ValType}

// External Types
pub(super) trait ExternType {}
impl ExternType for Func {}
impl ExternType for TableType {}
impl ExternType for MemType {}

// Import Types
pub(super) struct Import {
    module: String,
    name: String,
    desc: ImportDesc
}

pub(super) enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType)
}

// Export Types
pub(super) struct Export {
    name: String,
    desc: ExportDesc
}

pub(super) enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx)
}