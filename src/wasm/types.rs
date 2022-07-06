use super::instructions::Expr;

#[derive(Clone, Copy, std::fmt::Debug)]
pub(super) enum ValType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    V128,
    AnyNum,
    NullFuncRef,
    FuncRef,
    NullExternRef,
    ExternRef,
    AnyRef,
    Vec128,
    VecI8x16,
    VecI16x8,
    VecI32x4,
    VecI64x2,
    VecF32x4,
    VecF64x2,
    AnyVec,
    Any,
    Unknown
}
impl ValType {
    pub fn is_num(self) -> bool {
        match self {
            ValType::AnyNum => true,
            ValType::I8 => true,
            ValType::I16 => true,
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
            ValType::AnyVec => true,
            ValType::Vec128 => true,
            ValType::VecI8x16 => true,
            ValType::VecI16x8 => true,
            ValType::VecI32x4 => true,
            ValType::VecI64x2 => true,
            ValType::VecF32x4 => true,
            ValType::VecF64x2 => true,
            ValType::Unknown => true,
            _ => false
        }
    }

    pub fn is_ref(self) -> bool {
        match self {
            ValType::AnyRef => true,
            ValType::NullFuncRef => true,
            ValType::FuncRef => true,
            ValType::NullExternRef => true,
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
            ValType::Any => true,
            ValType::AnyNum => true,
            ValType::Unknown => true,
            _ => self.is_type(t)
        }
    }
}
impl Val for dyn VecType {
    fn is_type(&self, t: ValType) -> bool {
        match t {
            ValType::Any => true,
            ValType::AnyVec => true,
            ValType::Unknown => true,
            _ => self.is_type(t)
        }
    }
}
impl Val for RefType {
    fn is_type(&self, t: ValType) -> bool {
        match t {
            ValType::Any => true,
            ValType::AnyRef => true,
            ValType::Unknown => true,
            ValType::FuncRef => matches!(self, RefType::Ref(ValType::FuncRef, _)) || matches!(self, RefType::NullRef(ValType::FuncRef)),
            ValType::ExternRef => matches!(self, RefType::Ref(ValType::ExternRef, _)) || matches!(self, RefType::NullRef(ValType::ExternRef)),
            _ => false
        }
    }
}

pub(super) type ResultType = Vec<ValType>;

// Num Types
pub(super) trait NumType {
    fn is_type(&self, t: ValType) -> bool;
}
impl NumType for I8 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::I8) }
}
impl NumType for I16 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::I16) }
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

// Reference Types
pub(super) enum RefType {
    NullRef(ValType),
    Ref(ValType, TableIdx),
}

// Vector Types
pub(super) trait VecType {
    fn is_type(&self, t: ValType) -> bool;
    fn bits(&self) -> u128;
}

impl VecType for Vec128 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::Vec128) }
    fn bits(&self) -> u128 { self.data }
}

impl VecType for VecI8x16 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecI8x16) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 120) +
        ((self.data[1] as u128) << 112) +
        ((self.data[2] as u128) << 104) +
        ((self.data[3] as u128) << 96) +
        ((self.data[4] as u128) << 88) +
        ((self.data[5] as u128) << 80) +
        ((self.data[6] as u128) << 72) +
        ((self.data[7] as u128) << 64) +
        ((self.data[8] as u128) << 56) +
        ((self.data[9] as u128) << 48) +
        ((self.data[10] as u128) << 40) +
        ((self.data[11] as u128) << 32) +
        ((self.data[12] as u128) << 24) +
        ((self.data[13] as u128) << 16) +
        ((self.data[14] as u128) << 8) +
        ((self.data[15] as u128) << 0)
    }
}

impl VecType for VecI16x8 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecI16x8) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 112) +
        ((self.data[1] as u128) << 96) +
        ((self.data[2] as u128) << 80) +
        ((self.data[3] as u128) << 64) +
        ((self.data[4] as u128) << 48) +
        ((self.data[5] as u128) << 32) +
        ((self.data[6] as u128) << 16) +
        ((self.data[7] as u128) << 0)
    }
}

impl VecType for VecI32x4 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecI32x4) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecI64x2 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecI64x2) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 64) +
        ((self.data[1] as u128) << 0)
    }
}

impl VecType for VecF32x4 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecF32x4) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecF64x2 {
    fn is_type(&self, t: ValType) -> bool { matches!(t, ValType::VecF64x2) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 64) +
        ((self.data[1] as u128) << 0)
    }
}

pub(super) struct Vec128 {data: u128}
pub(super) struct VecI8x16 {data: [u8; 16]}
pub(super) struct VecI16x8 {data: [u16; 8]}
pub(super) struct VecI32x4 {data: [u32; 4]}
pub(super) struct VecI64x2 {data: [u64; 2]}
pub(super) struct VecF32x4 {data: [f32; 4]}
pub(super) struct VecF64x2 {data: [f64; 2]}

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
pub(super) struct TableType {limits: Limits, reftype: RefType}

pub(super) struct Elem {reftype: RefType, init: Vec<Expr>, mode: ElemMode}
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
pub(super) enum GlobalType {
    Var,
    Const
}

pub(super) struct Global {globaltype: GlobalType, valtype: ValType}

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